use std::{ borrow::BorrowMut, cell::RefCell, time::Duration };

use api::{
    AuthorizedCallerRequest,
    ClaimConsentRequest,
    ClaimConsentResponse,
    ConsentListResponse,
    CreateConsentResponse,
    EmrHeaderWithStatus,
    EmrListConsentRequest,
    EmrListConsentResponse,
    EmrListPatientRequest,
    EmrListPatientResponse,
    FinishSessionRequest,
    GetPatientInfoBySessionRequest,
    GetPatientInfoResponse,
    IsConsentClaimedRequest,
    IsConsentClaimedResponse,
    IssueRequest,
    PatientListResponse,
    PatientWithNikAndSession,
    PingResult,
    ReadEmrByIdRequest,
    ReadEmrSessionRequest,
    RegisterPatientRequest,
    RevokeConsentRequest,
    SearchPatientRequest,
    SearchPatientResponse,
    UpdateEmrRegistryRequest,
    UpdateInitialPatientInfoRequest,
    UpdateRequest,
};
use candid::{ Decode, Encode };
use canister_common::{
    common::{ guard::verified_caller, PrincipalBytes },
    id_generator::IdGenerator,
    log,
    mmgr::MemoryManager,
    opaque_metrics,
    random::CanisterRandomSource,
    register_log,
    stable::{ Candid, Memory, Stable, ToStable },
    statistics::{ self, traits::OpaqueMetrics },
};
use config::CanisterConfig;
use declarations::{ emr_registry::ReadEmrByIdResponse, provider_registry::GetProviderBatchRequest };

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
mod log;

type State = canister_common::common::State<
    registry::PatientRegistry,
    Cell<Stable<CanisterConfig, Candid>, Memory>,
    ()
>;

register_log!("patient");

// change this if you want to change the interval of the metrics collection
const METRICS_INTERVAL: Duration = Duration::from_secs(60 * 5); // 5 minutes

thread_local! {
    pub static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
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
    let mem = with_state(|s| s.memory_manager.get_memory::<_, UpgradeMemory>(|mem| mem));

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

#[ic_cdk::update]
async fn get_trusted_origins() -> Vec<String> {
    vec![
        // Origins should be in the format defined by the Window.postMessage method (https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage#the_dispatched_event)
        String::from("http://bw4dl-smaaa-aaaaa-qaacq-cai.localhost:4943"), // to be replaced with your frontend origin(s)
        // TODO: make a config out of this
        String::from("http://localhost:3000")
    ]
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

#[ic_cdk::query(guard = "only_patient", composite = true)]
async fn emr_list_patient(req: EmrListPatientRequest) -> EmrListPatientResponse {
    let caller = verified_caller().unwrap();
    let nik = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();

    let emrs = with_state(move |s|
        s.registry.emr_binding_map.emr_list(&nik, req.page, req.limit)
    ).unwrap();

    let provider_registry = with_state(|s| s.config.get().provider_registry());

    let providers = emrs
        .iter()
        .map(|header| header.provider_id.to_string())
        .collect::<Vec<_>>();

    let providers = provider_registry
        .get_provider_batch(GetProviderBatchRequest { ids: providers }).await
        .expect("failed to get providers info")
        .0.providers.into_iter()
        .map(|provider| {
            match provider {
                declarations::provider_registry::Provider::V1(provider) =>
                    provider.display_name.try_into().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    emrs.into_iter()
        .zip(providers.into_iter())

        .map(|(header, providers)| {
            let status = with_state(|s|
                s.registry.header_status_map
                    .get(&header)
                    .expect("issued emr must have valid status")
            );
            EmrHeaderWithStatus::new(header, status, providers)
        })
        .collect::<Vec<_>>()
        .into()
}

#[ic_cdk::update(guard = "only_provider_registry")]
fn notify_issued(req: IssueRequest) {
    with_state_mut(|s| s.registry.issue_for(req.header.user_id.clone(), req.header)).unwrap();
}

#[ic_cdk::update(guard = "only_provider_registry")]
fn notify_updated(req: UpdateRequest) {
    with_state_mut(|s| s.registry.header_status_map.update(req.header)).unwrap();
}

// TODO : unsafe, anybody can register as a patient and bind to any NIK, should discuss how do we gate this properly.
// probably best to only allow this be called from the frontend canister(todo)
#[ic_cdk::update]
fn register_patient(req: RegisterPatientRequest) {
    let owner = verified_caller().unwrap();
    with_state_mut(|s| s.registry.owner_map.bind(owner, req.nik)).unwrap()
}

// TODO : optimize this, this is a very expensive operation
#[ic_cdk::query]
fn patient_list() -> PatientListResponse {
    let caller = verified_caller().unwrap();
    let caller = PrincipalBytes::from(caller).into();

    if !ConsentsApi::is_session_user(&caller) {
        ic_cdk::trap("only session user can call this method");
    }

    let caller = caller.into_inner().into();

    let consents = ConsentsApi::user_list_with_consent(&caller);

    consents
        .into_iter()
        .map(|c| {
            let nik = c.nik;
            let session_id = c.session_id.expect("claimed session must have session id");
            let patient = with_state(|s|
                s.registry.get_patient_info(nik.clone()).expect("patient not found")
            );

            PatientWithNikAndSession::new(patient, nik, session_id)
        })
        .collect::<Vec<_>>()
        .into()
}

#[ic_cdk::query]
fn search_patient(req: SearchPatientRequest) -> SearchPatientResponse {
    let caller = verified_caller().unwrap();
    let caller = PrincipalBytes::from(caller).into();

    if !ConsentsApi::is_session_user(&caller) {
        ic_cdk::trap("only session user can call this method");
    }

    let caller = caller.into_inner().into();

    let consents = ConsentsApi::user_list_with_consent(&caller);

    consents
        .into_iter()
        .find(|c| c.nik == req.nik)
        .map(|c| {
            let nik = c.nik;
            let session_id = c.session_id.expect("claimed session must have session id");
            let patient = with_state(|s|
                s.registry.get_patient_info(nik.clone()).expect("patient not found")
            );

            PatientWithNikAndSession::new(patient, nik, session_id)
        })
        .expect("patient not found")
        .into()
}

#[ic_cdk::query(guard = "only_patient")]
fn is_consent_claimed(req: IsConsentClaimedRequest) -> IsConsentClaimedResponse {
    let caller = verified_caller().unwrap();
    let patient = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();

    if let Some(consent) = ConsentsApi::resolve_session_with_code(&req.code, &patient) {
        IsConsentClaimedResponse {
            claimed: consent.claimed,
            info: Some(consent),
        }
    } else {
        Default::default()
    }
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
) {
    canistergeek_ic_rust::update_information(request);
}

// TODO : make all emr list is available and user does not have to choose what emr to share, share everything by default
#[ic_cdk::update(guard = "only_patient")]
async fn create_consent() -> CreateConsentResponse {
    let owner = verified_caller().unwrap();
    let owner = with_state(|s| s.registry.owner_map.get_nik(&owner))
        .unwrap()
        .into_inner();

    ConsentsApi::generate_consent(owner).into()
}

#[ic_cdk::query(composite = true)]
async fn read_emr_with_session(
    req: ReadEmrSessionRequest
) -> crate::declarations::emr_registry::ReadEmrByIdResponse {
    let caller = verified_caller().unwrap();
    let registry = with_state(|s| s.config.get().emr_registry());
    ConsentsApi::read_emr_with_session(&req.session_id, req.args, registry, &caller).await.unwrap()
}

#[ic_cdk::query(composite = true)]
async fn emr_list_with_session(req: EmrListConsentRequest) -> EmrListConsentResponse {
    let caller = verified_caller().unwrap();
    let consent = ConsentsApi::resolve_session(&req.session_id, &caller).expect("invalid session");
    let nik = consent.nik;
    let info = with_state(|s| s.registry.info_map.get(nik.clone())).unwrap();

    let emrs = with_state(|s|
        s.registry.emr_binding_map.emr_list(&nik, req.page, req.limit)
    ).unwrap();

    let provider_registry = with_state(|s| s.config.get().provider_registry());

    let providers = emrs
        .iter()
        .map(|header| header.provider_id.to_string())
        .collect::<Vec<_>>();

    let providers = provider_registry
        .get_provider_batch(GetProviderBatchRequest { ids: providers }).await
        .expect("failed to get providers info")
        .0.providers.into_iter()
        .map(|provider| {
            match provider {
                declarations::provider_registry::Provider::V1(provider) =>
                    provider.display_name.try_into().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let emrs = emrs
        .into_iter()
        .zip(providers.into_iter())
        .map(move |(header, providers)| {
            let status = with_state(|s|
                s.registry.header_status_map
                    .get(&header)
                    .expect("issued emr must have valid status")
            );

            EmrHeaderWithStatus::new(header, status, providers)
        })
        .collect::<Vec<_>>()
        .into();

    EmrListConsentResponse::new(emrs, info.name().to_owned())
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
async fn derive_encryption_key_for_self() {
    todo!()
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
fn update_initial_patient_info(req: UpdateInitialPatientInfoRequest) {
    let caller = verified_caller().unwrap();

    with_state_mut(|s| s.registry.update_patient_info(caller, req.info.into())).unwrap()
}

#[ic_cdk::query]
fn get_patient_info_with_consent(req: GetPatientInfoBySessionRequest) -> GetPatientInfoResponse {
    let caller = verified_caller().unwrap();
    let consent = ConsentsApi::resolve_session(&req.session_id, &caller).expect("invalid session");
    let patient = with_state(|s| s.registry.get_patient_info(consent.nik.clone())).unwrap();
    GetPatientInfoResponse::new(patient, consent.nik)
}

#[ic_cdk::query(guard = "only_patient")]
fn get_patient_info() -> GetPatientInfoResponse {
    let caller = verified_caller().unwrap();
    let (patient, nik) = with_state(|s|
        s.registry.get_patient_info_with_principal(caller)
    ).unwrap();

    GetPatientInfoResponse::new(patient, nik)
}

#[ic_cdk::update(guard = "only_patient")]
fn revoke_consent(req: RevokeConsentRequest) {
    for code in req.codes {
        ConsentsApi::revoke_consent(&code);
    }
}

#[ic_cdk::update]
fn finish_session(req: FinishSessionRequest) {
    let caller = verified_caller().unwrap();

    ConsentsApi::finish_sesion(&req.session_id, &caller)
}

// TODO : move this into provider registry
#[ic_cdk::update]
fn claim_consent(req: ClaimConsentRequest) -> ClaimConsentResponse {
    let caller = verified_caller().unwrap();

    ConsentsApi::claim_consent(&req.code, caller)
        .expect("consent already claimed or does not exists")
        .into()
}

#[ic_cdk::query(guard = "only_patient")]
fn consent_list() -> ConsentListResponse {
    let caller = verified_caller().unwrap();
    let patient = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();
    let consents = ConsentsApi::list_consent_with_patient(&patient);

    consents.into()
}

ic_cdk::export_candid!();
