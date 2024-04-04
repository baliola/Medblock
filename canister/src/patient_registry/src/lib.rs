use std::{ borrow::BorrowMut, cell::RefCell, time::Duration };

use api::{
    AuthorizedCallerRequest,
    ClaimConsentRequest,
    ClaimConsentResponse,
    CreateConsentRequest,
    CreateConsentResponse,
    DeriveSecretKeyRequest,
    DeriveSecretKeyResponse,
    DeriveVerificationKeyRequest,
    DeriveVerificationKeyResponse,
    EmrListConsentRequest,
    EmrListConsentResponse,
    EmrListPatientRequest,
    EmrListPatientResponse,
    FinishSessionRequest,
    IssueRequest,
    PingResult,
    ReadEmrByIdRequest,
    ReadEmrSessionRequest,
    ReadEmrSessionResponse,
    RegisterPatientRequest,
    RevokeConsentRequest,
    UpdateEmrRegistryRequest,
};
use candid::{ Decode, Encode };
use canister_common::{
    common::guard::verified_caller,
    id_generator::IdGenerator,
    log,
    mmgr::MemoryManager,
    opaque_metrics,
    random::CanisterRandomSource,
    register_log,
    stable::{ Candid, Memory, Stable },
    statistics::{ self, traits::OpaqueMetrics },
};
use config::CanisterConfig;
use declarations::emr_registry::{ self, ReadEmrByIdResponse };
use encryption::vetkd;
use ic_cdk::api::management_canister::main::{CanisterIdRecord, CanisterSettings, UpdateSettingsArgument};
use ic_stable_structures::Cell;
use memory::UpgradeMemory;
use registry::PatientRegistry;

use crate::consent::ConsentsApi;

mod registry;
mod memory;
mod declarations;
mod api;
mod config;
mod encryption;
mod consent;

type State = canister_common::common::State<
    registry::PatientRegistry,
    Cell<Stable<CanisterConfig, Candid>, Memory>,
    ()
>;

register_log!("patient");

// change this if you want to change the interval of the metrics collection
const METRICS_INTERVAL: Duration = Duration::from_secs(60 * 5); // 5 minutes

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
    static ID_GENERATOR: RefCell<Option<IdGenerator<CanisterRandomSource>>> = const {
        RefCell::new(None)
    };
}

/// A helper method to read the state.
///
/// Precondition: the state is already initialized.
pub fn with_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(cell.borrow().as_ref().expect("state not initialized")))
}

/// A helper method to read the id generator.
///
/// Precondition: the id generator is already initialized.
pub fn with_id_generator_mut<R>(f: impl FnOnce(&mut IdGenerator<CanisterRandomSource>) -> R) -> R {
    ID_GENERATOR.with(|cell| f(cell.borrow_mut().as_mut().expect("id generator not initialized")))
}

/// A helper method to mutate the state.
///
/// Precondition: the state is already initialized.
pub fn with_state_mut<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(cell.borrow_mut().as_mut().expect("state not initialized")))
}

// guard function
fn only_canister_owner() -> Result<(), String> {
    let caller = verified_caller()?;

    match ic_cdk::api::is_controller(&caller) {
        true => Ok(()),
        false => Err("only canister controller can call this method".to_string()),
    }
}

// guard function
fn only_patient() -> Result<(), String> {
    let caller = verified_caller()?;

    match with_state(|s| s.registry.owner_map.is_valid_owner(&caller)) {
        true => Ok(()),
        false => Err("only patient can call this method".to_string()),
    }
}
// guard function
fn only_provider_registry() -> Result<(), String> {
    let caller = verified_caller()?;

    match with_state(|s| s.config.get().is_provider_registry(&caller)) {
        true => Ok(()),
        false => Err("only provider registry can call this method".to_string()),
    }
}

// guard function
fn only_authorized_metrics_collector() -> Result<(), String> {
    let caller = verified_caller()?;

    with_state(|s| {
        if !s.config.get().is_authorized_metrics_collector(&caller) {
            return Err("only authorized metrics collector can call this method".to_string());
        }

        Ok(())
    })
}

fn init_state() -> State {
    let memory_manager = MemoryManager::init();

    State {
        registry: PatientRegistry::init(&memory_manager),
        config: CanisterConfig::init(&memory_manager),
        freeze_threshold: (),
        memory_manager,
    }
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    serialize_canister_metrics();
}

fn serialize_canister_metrics() {
    let monitor_stable_data = canistergeek_ic_rust::monitor::pre_upgrade_stable_data();
    let logger_stable_data = canistergeek_ic_rust::logger::pre_upgrade_stable_data();

    let mut mem = with_state(|s| s.memory_manager.get_memory::<_, UpgradeMemory>(|mem| mem));
    let Ok(encoded) = Encode!(&monitor_stable_data, &logger_stable_data) else {
        return;
    };

    let encoded_len = encoded.len();
    let mut writer = ic_stable_structures::writer::Writer::new(&mut mem, 0);

    let write = writer.write(&encoded_len.to_le_bytes());
    match write {
        Ok(_) => {
            log!("encoded canister metrics length written successfully : {} bytes", encoded_len);
        }

        Err(e) => {
            log!("OOM ERROR: failed to write encoded canister metrics length {:?}", e);
        }
    }

    let write = writer.write(&encoded);
    match write {
        Ok(_) => {
            log!("encoded canister metrics written");
        }
        Err(e) => {
            log!("OOM ERROR: failed to write encoded canister metrics {:?}", e);
        }
    }
}

fn start_collect_metrics_job() {
    ic_cdk_timers::set_timer_interval(METRICS_INTERVAL, || {
        log!("updating metrics");

        canistergeek_ic_rust::update_information(
            canistergeek_ic_rust::api_type::UpdateInformationRequest {
                metrics: Some(canistergeek_ic_rust::api_type::CollectMetricsRequestType::force),
            }
        );

        log!("metrics updated");
    });
}

fn deserialize_canister_metrics() {
    let mut mem = with_state(|s| s.memory_manager.get_memory::<_, UpgradeMemory>(|mem| mem));

    let mut reader = ic_stable_structures::reader::Reader::new(&mem, 0);

    let mut len_buf = [0; 4];
    let read_len = reader.read(&mut len_buf).unwrap();
    log!("encoded canister metrics length: {:?} bytes", read_len);

    let mut state_buf = vec![0; u32::from_le_bytes(len_buf) as usize];
    let read_len = reader.read(&mut state_buf).unwrap();
    log!("readed encoded canister metrics length: {:?} bytes", read_len);

    let (monitor_stable_data, logger_stable_data) =
        Decode!(&state_buf, (u8, std::collections::BTreeMap<u32, canistergeek_ic_rust::monitor::data_type::DayData>), (u8, canistergeek_ic_rust::logger::LogMessageStorage)).unwrap();

    canistergeek_ic_rust::monitor::post_upgrade_stable_data(monitor_stable_data);
    canistergeek_ic_rust::logger::post_upgrade_stable_data(logger_stable_data);
}

fn initialize_id_generator() {
    ic_cdk_timers::set_timer(Duration::from_secs(3), || {
        ic_cdk::spawn(async move {
            let rng = CanisterRandomSource::new().await;
            let id_generator = IdGenerator::new(rng);

            ID_GENERATOR.replace(Some(id_generator));

            log!("id generator initialized");
        })
    });
}

// TODO : implement scope guard for inter-canister calls
fn initialize() {
    let state = init_state();
    STATE.replace(Some(state));
    log!("canister state initialized");
    initialize_id_generator();
    ConsentsApi::init();
    start_collect_metrics_job();
}

#[ic_cdk::inspect_message]
fn inspect_message() {
    verified_caller().expect("caller is not verified");

    match only_canister_owner().is_ok() || only_patient().is_ok() {
        true => ic_cdk::api::call::accept_message(),
        false => ic_cdk::trap("unauthorized"),
    }
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    initialize();
    deserialize_canister_metrics();
}

#[ic_cdk::init]
fn canister_init() {
    initialize()
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn remove_authorized_metrics_collector(req: AuthorizedCallerRequest) {
    with_state_mut(|s| {
        let mut config = s.config.get().to_owned();

        config.remove_authorized_metrics_collector(req.caller);

        s.config.set(config);
    });
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn add_authorized_metrics_collector(req: AuthorizedCallerRequest) {
    with_state_mut(|s| {
        let mut config = s.config.get().to_owned();

        config.add_authorized_metrics_collector(req.caller);

        s.config.set(config);
    });
}

#[ic_cdk::query(composite = true, guard = "only_patient")]
async fn read_emr_by_id(req: ReadEmrByIdRequest) -> ReadEmrByIdResponse {
    let user = verified_caller().unwrap();
    let args = with_state(|s| s.registry.construct_args_read_emr(req, &user)).unwrap();

    let registry = with_state(|s| s.config.get().emr_registry());
    PatientRegistry::do_call_read_emr(args, registry).await
}

fn emr_list_patient(req: EmrListPatientRequest) -> EmrListPatientResponse {
    let caller = verified_caller().unwrap();
    let nik = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();

    with_state(move |s| s.registry.emr_binding_map.emr_list(&nik, req.page, req.limit))
        .unwrap()
        .into()
}

#[ic_cdk::update(guard = "only_provider_registry")]
fn notify_issued(req: IssueRequest) {
    with_state_mut(|s|
        s.registry.emr_binding_map.issue_for(req.header.user_id.clone(), req.header)
    ).unwrap();
}

// TODO : unsafe, anybody can register as a patient and bind to any NIK, should discuss how do we gate this properly.
// probably best to only allow this be called from the frontend canister(todo)
#[ic_cdk::update]
fn register_patient(req: RegisterPatientRequest) {
    let owner = verified_caller().unwrap();
    with_state_mut(|s| s.registry.owner_map.bind(owner, req.nik)).unwrap()
}

#[ic_cdk::query(composite = true)]
async fn ping() -> PingResult {
    let emr_registry = with_state(|s| s.config.get().emr_registry());
    let emr_registry_status = emr_registry.ping().await.is_ok();

    PingResult {
        emr_registry_status,
    }
}

#[ic_cdk::query(guard = "only_authorized_metrics_collector")]
fn metrics() -> String {
    with_state(|s| {
        [
            ConsentsApi::metrics(),
            opaque_metrics!(s.registry),
            OpaqueMetrics::measure(&**s.config.get()),
            statistics::canister::BlockchainMetrics::measure(),
            statistics::canister::MemoryStatistics::measure(),
        ].join("\n")
    })
}

#[ic_cdk::query(guard = "only_authorized_metrics_collector", name = "getCanistergeekInformation")]
pub async fn canister_geek_metrics(
    request: canistergeek_ic_rust::api_type::GetInformationRequest
) -> canistergeek_ic_rust::api_type::GetInformationResponse<'static> {
    canistergeek_ic_rust::get_information(request)
}

#[ic_cdk::update(
    guard = "only_authorized_metrics_collector",
    name = "updateCanistergeekInformation"
)]
pub async fn update_canistergeek_information(
    request: canistergeek_ic_rust::api_type::UpdateInformationRequest
) -> () {
    canistergeek_ic_rust::update_information(request);
}

#[ic_cdk::update(guard = "only_patient")]
async fn create_consent(req: CreateConsentRequest) -> CreateConsentResponse {
    let owner = verified_caller().unwrap();
    let owner = with_state(|s| s.registry.owner_map.get_nik(&owner))
        .unwrap()
        .into_inner();

    ConsentsApi::generate_consent(owner, req.allowed).into()
}

#[ic_cdk::query(composite = true)]
async fn read_emr_with_session(
    req: ReadEmrSessionRequest
) -> crate::declarations::emr_registry::ReadEmrByIdResponse {
    let caller = verified_caller().unwrap();
    let registry = with_state(|s| s.config.get().emr_registry());
    ConsentsApi::read_emr_with_session(&req.session_id, req.args, registry, &caller).await.unwrap()
}

#[ic_cdk::query]
async fn emr_list_with_session(req: EmrListConsentRequest) -> EmrListConsentResponse {
    let caller = verified_caller().unwrap();
    ConsentsApi::emr_list_with_session(&req.session_id, &caller).unwrap().into()
}

#[cfg(feature = "vetkd")]
#[ic_cdk::update]
/// Derive the encryption key with the session id securely transported by encrypting the decryption key, used to decrypt emr
async fn derive_encryption_key_with_session(
    req: DeriveSecretKeyRequest
) -> DeriveSecretKeyResponse {
    let consent = ConsentsApi::resolve_session(&req.session_id).expect("session not found");
    vetkd::EncryptionApi::encrypted_emr_decryption_key(req.transport_key, &consent.nik).await.into()
}

#[cfg(feature = "vetkd")]
#[ic_cdk::update]
/// Derive the encryption verification key with the session id, used to verify the encrypted emr decryption key
async fn derive_encryption_verification_key_with_session(
    req: DeriveVerificationKeyRequest
) -> DeriveVerificationKeyResponse {
    let consent = ConsentsApi::resolve_session(&req.session_id).expect("session not found");
    vetkd::EncryptionApi::verification_key_for(&consent.nik).await.into()
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn update_emr_registry_principal(req: UpdateEmrRegistryRequest) {
    with_state_mut(|s| {
        let mut config = s.config.get().to_owned();

        config.update_default_emr_registry_principal(req.principal);

        match s.config.set(config) {
            Ok(_) => (),
            Err(e) => ic_cdk::trap(&format!("failed to update emr registry principal: {:?}", e)),
        }
    })
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn update_provider_registry_principal(req: UpdateEmrRegistryRequest) {
    with_state_mut(|s| {
        let mut config = s.config.get().to_owned();

        config.update_provider_registry_principal(req.principal);

        match s.config.set(config) {
            Ok(_) => (),
            Err(e) => ic_cdk::trap(&format!("failed to update emr registry principal: {:?}", e)),
        }
    })
}


#[ic_cdk::update(guard = "only_patient")]
fn revoke_consent(req: RevokeConsentRequest) {
    ConsentsApi::revoke_consent(&req.code);
}

#[ic_cdk::update]
fn finish_session(req: FinishSessionRequest) {
    let caller = verified_caller().unwrap();

    ConsentsApi::finish_sesion(&req.session_id, &caller)
}

#[ic_cdk::update]
fn claim_consent(req: ClaimConsentRequest) -> ClaimConsentResponse {
    let caller = verified_caller().unwrap();

    ConsentsApi::claim_consent(&req.code, caller)
        .expect("consent already claimed or does not exists")
        .into()
}

ic_cdk::export_candid!();
