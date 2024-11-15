use std::{borrow::BorrowMut, cell::RefCell, str::FromStr, time::Duration};

use api::{
    AddGroupMemberRequest, AuthorizedCallerRequest, BindAdminRequest, ClaimConsentRequest,
    ClaimConsentResponse, ConsentListResponse, CreateConsentResponse, CreateGroupRequest,
    CreateGroupResponse, EmrHeaderWithStatus, EmrListConsentRequest, EmrListConsentResponse,
    EmrListPatientRequest, EmrListPatientResponse, FinishSessionRequest,
    GetPatientInfoBySessionRequest, GetPatientInfoResponse, GetUserGroupsResponse,
    GrantGroupAccessRequest, IsConsentClaimedRequest, IsConsentClaimedResponse, IssueRequest,
    LeaveGroupRequest, LogResponse, PatientListAdminResponse, PatientListResponse, PatientWithNik,
    PatientWithNikAndSession, PingResult, ReadEmrByIdRequest, ReadEmrSessionRequest,
    RegisterPatientRequest, RevokeConsentRequest, RevokeGroupAccessRequest,
    SearchPatientAdminResponse, SearchPatientRequest, SearchPatientResponse,
    UpdateEmrRegistryRequest, UpdateInitialPatientInfoRequest, UpdateKycStatusRequest,
    UpdateKycStatusResponse, UpdateRequest, ViewGroupMemberEmrInformationRequest,
};
use candid::{Decode, Encode};
use canister_common::{
    common::{guard::verified_caller, AsciiRecordsKey, ProviderId},
    id_generator::IdGenerator,
    log,
    mmgr::MemoryManager,
    opaque_metrics,
    random::CanisterRandomSource,
    register_log,
    stable::{Candid, Memory, Stable, ToStable},
    statistics::{self, traits::OpaqueMetrics},
};
use config::CanisterConfig;
use consent::ConsentCode;
use declarations::{emr_registry::ReadEmrByIdResponse, provider_registry::GetProviderBatchRequest};

use ic_stable_structures::Cell;
use log::PatientLog;
use memory::UpgradeMemory;
use registry::{Group, GroupId, PatientRegistry, NIK};

use crate::consent::ConsentsApi;

mod api;
mod config;
mod consent;
mod declarations;
mod encryption;
mod log;
mod memory;
mod registry;

pub struct State {
    pub registry: registry::PatientRegistry,
    pub config: Cell<Stable<CanisterConfig, Candid>, Memory>,
    pub memory_manager: MemoryManager,
    pub patient_log: PatientLog,
}

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
    ID_GENERATOR.with(|cell| {
        f(cell
            .borrow_mut()
            .as_mut()
            .expect("id generator not initialized"))
    })
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

// guard function
fn only_admin() -> Result<(), String> {
    let caller = verified_caller()?;

    match with_state(|s| s.registry.admin_map.is_valid_admin(&caller)) {
        true => Ok(()),
        false => Err("only admin can call this method".to_string()),
    }
}

// guard function
fn only_controller() -> Result<(), String> {
    let caller = verified_caller()?;

    match ic_cdk::api::is_controller(&caller) {
        true => Ok(()),
        false => Err("only controller can call this method".to_string()),
    }
}

// guard function for only admin_or_controller
fn only_admin_or_controller() -> Result<(), String> {
    let caller = verified_caller()?;

    let is_admin = with_state(|s| s.registry.admin_map.is_valid_admin(&caller));
    let is_controller = ic_cdk::api::is_controller(&caller);

    if is_admin || is_controller {
        Ok(())
    } else {
        Err("only admin or controller can call this method".to_string())
    }
}

// guard function combination of only_admin and only_controller
fn only_admin_or_controller_or_patient() -> Result<(), String> {
    let caller = verified_caller()?;

    // Check if caller is either admin or controller
    let is_admin = with_state(|s| s.registry.admin_map.is_valid_admin(&caller));
    let is_controller = ic_cdk::api::is_controller(&caller);
    let is_patient = with_state(|s| s.registry.owner_map.is_valid_owner(&caller));

    if is_admin || is_controller || is_patient {
        Ok(())
    } else {
        Err("only admin or controller or patient can call this method".to_string())
    }
}

fn init_state() -> State {
    let memory_manager = MemoryManager::init();

    State {
        registry: PatientRegistry::init(&memory_manager),
        config: CanisterConfig::init(&memory_manager),
        patient_log: PatientLog::init(&memory_manager),
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
            log!(
                "encoded canister metrics length written successfully : {} bytes",
                encoded_len
            );
        }

        Err(e) => {
            log!(
                "OOM ERROR: failed to write encoded canister metrics length {:?}",
                e
            );
        }
    }

    let write = writer.write(&encoded);
    match write {
        Ok(_) => {
            log!("encoded canister metrics written");
        }
        Err(e) => {
            log!(
                "OOM ERROR: failed to write encoded canister metrics {:?}",
                e
            );
        }
    }
}

fn start_collect_metrics_job() {
    ic_cdk_timers::set_timer_interval(METRICS_INTERVAL, || {
        log!("updating metrics");

        canistergeek_ic_rust::update_information(
            canistergeek_ic_rust::api_type::UpdateInformationRequest {
                metrics: Some(canistergeek_ic_rust::api_type::CollectMetricsRequestType::force),
            },
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
    log!(
        "readed encoded canister metrics length: {:?} bytes",
        read_len
    );

    let (monitor_stable_data, logger_stable_data) = Decode!(
        &state_buf,
        (
            u8,
            std::collections::BTreeMap<u32, canistergeek_ic_rust::monitor::data_type::DayData>
        ),
        (u8, canistergeek_ic_rust::logger::LogMessageStorage)
    )
    .unwrap();

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
        String::from("http://localhost:3000"),
        String::from("https://demo-app.medblock.id"),
        String::from("https://demo-web.medblock.id"),
        String::from("https://dev-web.medblock.id"),
        String::from("https:/-app.medblock.id"),
        String::from("http://54.255.210.149:3001"),
        String::from("http://54.255.210.149:3000"),
        String::from("https://bwvkymxvy2stchdh.medblock.id"),
        String::from("https://bwvkymxvy2std2vi.medblock.id"),
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

    let emrs = with_state(move |s| {
        s.registry
            .emr_binding_map
            .emr_list(&nik, req.page, req.limit)
    })
    .unwrap();

    let provider_registry = with_state(|s| s.config.get().provider_registry());

    let providers = emrs
        .iter()
        .map(|header| header.provider_id.to_string())
        .collect::<Vec<_>>();

    let providers = provider_registry
        .get_provider_batch(GetProviderBatchRequest { ids: providers })
        .await
        .expect("failed to get providers info")
        .0
        .providers
        .into_iter()
        .map(|provider| match provider {
            declarations::provider_registry::Provider::V1(provider) => {
                provider.display_name.try_into().unwrap()
            }
        })
        .collect::<Vec<_>>();

    emrs.into_iter()
        .zip(providers.into_iter())
        .map(|(header, providers)| {
            let status = with_state(|s| {
                s.registry
                    .header_status_map
                    .get(&header)
                    .expect("issued emr must have valid status")
            });
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
    with_state_mut(|s| {
        s.patient_log.record(
            log::ActivityType::Updated,
            req.header.provider_id.clone(),
            req.header.user_id.clone(),
        )
    });
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
#[ic_cdk::query(composite = true)]
async fn patient_list() -> PatientListResponse {
    let caller = verified_caller().unwrap();
    let provider_registry = with_state(|s| s.config.get().provider_registry());
    let args = PatientRegistry::construct_get_provider_batch_args(vec![caller]);
    let provider = PatientRegistry::do_call_get_provider_batch(args, provider_registry).await;
    let provider: ProviderId = match provider.providers.first().unwrap() {
        declarations::provider_registry::Provider::V1(provider) => {
            provider.internal_id.clone().try_into().unwrap()
        }
    };

    if !ConsentsApi::is_session_user(provider.to_stable_ref()) {
        ic_cdk::trap("only session user can call this method");
    }

    let consents = ConsentsApi::user_list_with_consent(&provider);

    consents
        .into_iter()
        .map(|c| {
            let nik = c.nik;
            let session_id = c.session_id.expect("claimed session must have session id");
            let patient = with_state(|s| {
                s.registry
                    .get_patient_info(nik.clone())
                    .expect("patient not found")
            });

            PatientWithNikAndSession::new(patient, nik, session_id)
        })
        .collect::<Vec<_>>()
        .into()
}

// a patient list function for admins only
#[ic_cdk::query(guard = "only_admin_or_controller")]
async fn get_patient_list_admin() -> PatientListAdminResponse {
    // get all NIKs from the owner map
    let patients = with_state(|s| {
        s.registry
            .owner_map
            .get_all_nik()
            .iter()
            .map(|nik| {
                let nik = nik.clone().into_inner();
                let patient = s
                    .registry
                    .get_patient_info(nik.clone())
                    .expect("patient not found");

                PatientWithNik::new(patient, nik)
            })
            .collect::<Vec<_>>()
    });

    PatientListAdminResponse::from(patients)
}

#[ic_cdk::query(composite = true)]
async fn search_patient(req: SearchPatientRequest) -> SearchPatientResponse {
    let caller = verified_caller().unwrap();
    let provider_registry = with_state(|s| s.config.get().provider_registry());
    let args = PatientRegistry::construct_get_provider_batch_args(vec![caller]);
    let provider = PatientRegistry::do_call_get_provider_batch(args, provider_registry).await;
    let provider: ProviderId = match provider.providers.first().unwrap() {
        declarations::provider_registry::Provider::V1(provider) => {
            provider.internal_id.clone().try_into().unwrap()
        }
    };
    if !ConsentsApi::is_session_user(provider.to_stable_ref()) {
        ic_cdk::trap("only session user can call this method");
    }

    let consents = ConsentsApi::user_list_with_consent(&provider);

    consents
        .into_iter()
        .find(|c| c.nik == req.nik)
        .map(|c| {
            let nik = c.nik;
            let session_id = c.session_id.expect("claimed session must have session id");
            let patient = with_state(|s| {
                s.registry
                    .get_patient_info(nik.clone())
                    .expect("patient not found")
            });

            PatientWithNikAndSession::new(patient, nik, session_id)
        })
        .expect("patient not found")
        .into()
}

/// Search Patient for Admin UI
///
/// Description: This function is used to search for a patient by their NIK.
///
/// Parameters:
/// - req: SearchPatientRequest
///
/// Returns:
/// - SearchPatientAdminResponse
#[ic_cdk::query(guard = "only_admin_or_controller")]
fn search_patient_admin(req: SearchPatientRequest) -> SearchPatientAdminResponse {
    let patient = with_state(|s| s.registry.get_patient_info(req.nik.clone())).unwrap();

    let patient = PatientWithNik::new(patient, req.nik);

    SearchPatientAdminResponse::new(patient)
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
        ]
        .join("\n")
    })
}

#[ic_cdk::query(
    guard = "only_authorized_metrics_collector",
    name = "getCanistergeekInformation"
)]
pub async fn canister_geek_metrics(
    request: canistergeek_ic_rust::api_type::GetInformationRequest,
) -> canistergeek_ic_rust::api_type::GetInformationResponse<'static> {
    canistergeek_ic_rust::get_information(request)
}

#[ic_cdk::update(
    guard = "only_authorized_metrics_collector",
    name = "updateCanistergeekInformation"
)]
pub async fn update_canistergeek_information(
    request: canistergeek_ic_rust::api_type::UpdateInformationRequest,
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
    req: ReadEmrSessionRequest,
) -> crate::declarations::emr_registry::ReadEmrByIdResponse {
    let caller = verified_caller().unwrap();
    let provider_registry = with_state(|s| s.config.get().provider_registry());
    let args = PatientRegistry::construct_get_provider_batch_args(vec![caller]);
    let provider = PatientRegistry::do_call_get_provider_batch(args, provider_registry).await;
    let provider = match provider.providers.first().unwrap() {
        declarations::provider_registry::Provider::V1(provider) => {
            provider.internal_id.clone().try_into().unwrap()
        }
    };

    let registry = with_state(|s| s.config.get().emr_registry());
    ConsentsApi::read_emr_with_session(&req.session_id, req.args, registry, &provider)
        .await
        .unwrap()
}

#[ic_cdk::query(composite = true)]
async fn emr_list_with_session(req: EmrListConsentRequest) -> EmrListConsentResponse {
    let caller = verified_caller().unwrap();
    let provider_registry = with_state(|s| s.config.get().provider_registry());
    let args = PatientRegistry::construct_get_provider_batch_args(vec![caller]);
    let provider = PatientRegistry::do_call_get_provider_batch(args, provider_registry).await;
    let provider = match provider.providers.first().unwrap() {
        declarations::provider_registry::Provider::V1(provider) => {
            provider.internal_id.clone().try_into().unwrap()
        }
    };

    let consent =
        ConsentsApi::resolve_session(&req.session_id, &provider).expect("invalid session");
    let nik = consent.nik;
    let info = with_state(|s| s.registry.info_map.get(nik.clone())).unwrap();

    let emrs = with_state(|s| {
        s.registry
            .emr_binding_map
            .emr_list(&nik, req.page, req.limit)
    })
    .unwrap();

    let provider_registry = with_state(|s| s.config.get().provider_registry());

    let providers = emrs
        .iter()
        .map(|header| header.provider_id.to_string())
        .collect::<Vec<_>>();

    let providers = provider_registry
        .get_provider_batch(GetProviderBatchRequest { ids: providers })
        .await
        .expect("failed to get providers info")
        .0
        .providers
        .into_iter()
        .map(|provider| match provider {
            declarations::provider_registry::Provider::V1(provider) => {
                provider.display_name.try_into().unwrap()
            }
        })
        .collect::<Vec<_>>();

    let emrs = emrs
        .into_iter()
        .zip(providers.into_iter())
        .map(move |(header, providers)| {
            let status = with_state(|s| {
                s.registry
                    .header_status_map
                    .get(&header)
                    .expect("issued emr must have valid status")
            });

            EmrHeaderWithStatus::new(header, status, providers)
        })
        .collect::<Vec<_>>();

    EmrListConsentResponse::new(emrs, info.name().to_owned())
}

#[cfg(feature = "vetkd")]
#[ic_cdk::update]
/// Derive the encryption key with the session id securely transported by encrypting the decryption key, used to decrypt emr
async fn derive_encryption_key_with_session(
    req: DeriveSecretKeyRequest,
) -> DeriveSecretKeyResponse {
    let consent = ConsentsApi::resolve_session(&req.session_id).expect("session not found");
    vetkd::EncryptionApi::encrypted_emr_decryption_key(req.transport_key, &consent.nik)
        .await
        .into()
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
    req: DeriveVerificationKeyRequest,
) -> DeriveVerificationKeyResponse {
    let consent = ConsentsApi::resolve_session(&req.session_id).expect("session not found");
    vetkd::EncryptionApi::verification_key_for(&consent.nik)
        .await
        .into()
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

    with_state_mut(|s| s.registry.initial_patient_info(caller, req.info.into())).unwrap()
}

#[ic_cdk::query(composite = true)]
async fn get_patient_info_with_consent(
    req: GetPatientInfoBySessionRequest,
) -> GetPatientInfoResponse {
    let caller = verified_caller().unwrap();
    let provider_registry = with_state(|s| s.config.get().provider_registry());
    let args = PatientRegistry::construct_get_provider_batch_args(vec![caller]);
    let provider = PatientRegistry::do_call_get_provider_batch(args, provider_registry).await;
    let provider = match provider.providers.first().unwrap() {
        declarations::provider_registry::Provider::V1(provider) => {
            provider.internal_id.clone().try_into().unwrap()
        }
    };

    let consent =
        ConsentsApi::resolve_session(&req.session_id, &provider).expect("invalid session");
    let patient = with_state(|s| s.registry.get_patient_info(consent.nik.clone())).unwrap();
    GetPatientInfoResponse::new(patient, consent.nik)
}

#[ic_cdk::query(guard = "only_admin_or_controller_or_patient")]
fn get_patient_info() -> GetPatientInfoResponse {
    let caller = verified_caller().unwrap();
    let (patient, nik) =
        with_state(|s| s.registry.get_patient_info_with_principal(caller)).unwrap();

    GetPatientInfoResponse::new(patient, nik)
}

#[ic_cdk::update(guard = "only_patient")]
fn revoke_consent(req: RevokeConsentRequest) {
    for code in req.codes {
        let user_consent = ConsentsApi::consent(&code).expect("consent not found");
        ConsentsApi::revoke_consent(&code);

        if !user_consent.claimed && user_consent.session_user.is_none() {
            continue;
        }

        with_state_mut(|s| {
            s.patient_log.record(
                log::ActivityType::Revoked,
                user_consent.session_user.unwrap(),
                user_consent.nik,
            )
        });
    }
}

#[ic_cdk::query(guard = "only_patient")]
fn get_logs() -> LogResponse {
    let caller = verified_caller().unwrap();
    let nik = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();
    let logs = match with_state(|s| s.patient_log.get_logs(&nik)) {
        Some(logs) => logs.into_iter().map(|log| log.into_inner()).collect(),
        None => vec![],
    };

    LogResponse::new(logs)
}

#[ic_cdk::update]
async fn finish_session(req: FinishSessionRequest) {
    let caller = verified_caller().unwrap();
    let provider_registry = with_state(|s| s.config.get().provider_registry());
    let args = PatientRegistry::construct_get_provider_batch_args(vec![caller]);
    let provider = PatientRegistry::do_call_get_provider_batch(args, provider_registry).await;
    let provider = match provider.providers.first().unwrap() {
        declarations::provider_registry::Provider::V1(provider) => {
            provider.internal_id.clone().try_into().unwrap()
        }
    };

    ConsentsApi::finish_sesion(&req.session_id, &provider);
}

// TODO : move this into provider registry
#[ic_cdk::update]
async fn claim_consent(req: ClaimConsentRequest) -> ClaimConsentResponse {
    let caller = verified_caller().unwrap();
    let provider_registry = with_state(|s| s.config.get().provider_registry());
    let args = PatientRegistry::construct_get_provider_batch_args(vec![caller]);
    let provider = PatientRegistry::do_call_get_provider_batch(args, provider_registry).await;
    let provider: ProviderId = match provider.providers.first().unwrap() {
        declarations::provider_registry::Provider::V1(provider) => {
            provider.internal_id.clone().try_into().unwrap()
        }
    };

    let (session_id, nik) = ConsentsApi::claim_consent(&req.code, provider.clone())
        .expect("consent already claimed or does not exists");

    with_state_mut(|s| {
        s.patient_log
            .record(log::ActivityType::Accessed, provider, nik.clone())
    });

    let patient = with_state(|s| s.registry.get_patient_info(nik).unwrap())
        .name()
        .to_owned();

    ClaimConsentResponse::new(session_id, patient)
}

#[ic_cdk::query(guard = "only_patient")]
fn consent_list() -> ConsentListResponse {
    let caller = verified_caller().unwrap();
    let patient = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();
    let consents = ConsentsApi::list_consent_with_patient(&patient);

    consents.into()
}

// get patie \nt list only admin and comtrollret context

#[ic_cdk::update(guard = "only_admin")]
fn update_kyc_status(req: UpdateKycStatusRequest) -> UpdateKycStatusResponse {
    // get existing patient info
    let patient = with_state(|s| s.registry.get_patient_info(req.nik.clone())).unwrap();

    // create updated patient with new kyc status
    let mut updated_patient = patient.clone();
    updated_patient.update_kyc_status(req.kyc_status);

    // get the patient's principal to update their info
    let patient_principal = with_state(|s| s.registry.owner_map.get_principal(&req.nik)).unwrap();

    // update the patient info using their principal
    with_state_mut(|s| {
        s.registry
            .update_patient_info(patient_principal, updated_patient.clone())
    })
    .unwrap();

    UpdateKycStatusResponse::new(updated_patient)
}

#[ic_cdk::update(guard = "only_canister_owner")]
fn bind_admin(req: BindAdminRequest) {
    with_state_mut(|s| s.registry.admin_map.bind(req.principal, req.nik)).unwrap();
}

#[ic_cdk::update(guard = "only_patient")]
fn create_group(req: CreateGroupRequest) -> Result<CreateGroupResponse, String> {
    let caller = verified_caller().unwrap();
    let nik = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();

    let name =
        AsciiRecordsKey::<64>::new(req.name).map_err(|e| format!("Invalid group name: {}", e))?;

    Ok(with_state_mut(|s| s.registry.group_map.create_group(name, nik)).into())
}

#[ic_cdk::update(guard = "only_patient")]
fn add_group_member(req: AddGroupMemberRequest) -> Result<(), String> {
    let caller = verified_caller().unwrap();
    let leader_nik = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();

    let code = ConsentCode::from_text(&req.consent_code)
        .map_err(|e| format!("Invalid consent code: {}", e))?;

    let consent = ConsentsApi::consent(&code).ok_or("Consent not found")?;

    with_state_mut(|s| {
        s.registry
            .group_map
            .add_member(req.group_id, &leader_nik, consent.nik)
    })
    .map_err(|e| format!("Failed to add member: {:?}", e))
}

#[ic_cdk::update(guard = "only_patient")]
fn leave_group(req: LeaveGroupRequest) -> Result<(), String> {
    let caller = verified_caller().unwrap();
    let nik = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();

    with_state_mut(|s| s.registry.group_map.remove_member(req.group_id, &nik))
        .map_err(|e| format!("Failed to leave group: {:?}", e))
}

#[ic_cdk::query(guard = "only_patient")]
fn get_user_groups() -> GetUserGroupsResponse {
    let caller = verified_caller().unwrap();
    let nik = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();

    with_state(|s| s.registry.group_map.get_user_groups(&nik)).into()
}

#[ic_cdk::update(guard = "only_patient")]
fn grant_group_access(req: GrantGroupAccessRequest) -> Result<(), String> {
    let caller = verified_caller().unwrap();
    let granter_nik = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();

    log!("DEBUG granter_nik: {:?}", granter_nik);

    // Parse grantee NIK from string
    let grantee_nik = NIK::from_str(&req.grantee_nik.to_string())
        .map_err(|_| "Invalid grantee NIK format".to_string())?;

    log!("DEBUG grantee_nik: {:?}", grantee_nik);
    log!("DEBUG group_id: {:?}", req.group_id);

    // Verify both users are in the same group
    let group =
        with_state(|s| s.registry.group_map.get_group(req.group_id)).ok_or("Group not found")?;

    log!("DEBUG group members: {:?}", group.members);
    log!(
        "DEBUG granter in group: {:?}",
        group.members.contains(&granter_nik)
    );
    log!(
        "DEBUG grantee in group: {:?}",
        group.members.contains(&grantee_nik)
    );

    // Check both users are in the group
    if !group.members.contains(&granter_nik) {
        return Err("Granter is not a member of the group".to_string());
    }
    if !group.members.contains(&grantee_nik) {
        return Err("Grantee is not a member of the group".to_string());
    }

    // Grant EMR access to grantee
    with_state_mut(|s| {
        s.registry
            .group_access_map
            .grant_access(granter_nik, grantee_nik, req.group_id)
    })
    .map_err(|e| format!("Failed to grant EMR access: {:?}", e))
}

#[ic_cdk::update(guard = "only_patient")]
fn revoke_group_access(req: RevokeGroupAccessRequest) -> Result<(), String> {
    let caller = verified_caller().unwrap();
    let granter_nik = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();

    // parse grantee NIK from string
    let grantee_nik = NIK::from_str(&req.grantee_nik.to_string())
        .map_err(|_| "Invalid grantee NIK format".to_string())?;

    // revoke EMR access from grantee
    with_state_mut(|s| {
        s.registry
            .group_access_map
            .revoke_access(granter_nik, grantee_nik)
    })
    .map_err(|e| format!("Failed to revoke EMR access: {:?}", e))
}

#[ic_cdk::query(guard = "only_patient")]
async fn view_group_member_emr_information(
    req: ViewGroupMemberEmrInformationRequest,
) -> Result<EmrListPatientResponse, String> {
    // get caller's NIK
    let caller = verified_caller().unwrap();
    let viewer_nik = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();

    // parse target member's NIK from string
    let member_nik =
        NIK::from_str(&req.member_nik).map_err(|_| "Invalid member NIK format".to_string())?;

    // verify both users are in the same group
    let group =
        with_state(|s| s.registry.group_map.get_group(req.group_id)).ok_or("Group not found")?;

    if !group.members.contains(&viewer_nik) || !group.members.contains(&member_nik) {
        return Err("Both users must be members of the group".to_string());
    }

    // verify access has been granted
    let has_access = with_state(|s| {
        s.registry
            .group_access_map
            .has_access(&member_nik, &viewer_nik)
    });

    if !has_access {
        return Err("No access granted to view this member's EMR information".to_string());
    }

    // get member's EMRs with pagination
    let emrs = with_state(|s| {
        s.registry
            .emr_binding_map
            .emr_list(&member_nik, req.page as u8, req.limit as u8)
    })
    .map_err(|e| format!("Failed to get EMR list: {:?}", e))?;

    // get provider information for each EMR
    let provider_registry = with_state(|s| s.config.get().provider_registry());

    let providers = emrs
        .iter()
        .map(|header| header.provider_id.to_string())
        .collect::<Vec<_>>();

    let providers = provider_registry
        .get_provider_batch(GetProviderBatchRequest { ids: providers })
        .await
        .expect("failed to get providers info")
        .0
        .providers
        .into_iter()
        .map(|provider| match provider {
            declarations::provider_registry::Provider::V1(provider) => {
                provider.display_name.try_into().unwrap()
            }
        })
        .collect::<Vec<_>>();

    // combine EMR headers with provider information
    let emrs = emrs
        .into_iter()
        .zip(providers.into_iter())
        .map(|(header, provider)| {
            let status = with_state(|s| {
                s.registry
                    .header_status_map
                    .get(&header)
                    .expect("issued emr must have valid status")
            });
            EmrHeaderWithStatus::new(header, status, provider)
        })
        .collect::<Vec<_>>();

    Ok(EmrListPatientResponse::from(emrs))
}

ic_cdk::export_candid!();
