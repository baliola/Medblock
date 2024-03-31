use std::{ cell::RefCell, time::Duration };

use api::{
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
};
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
use ic_stable_structures::Cell;
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
    Ok(())
}

// guard function
fn only_patient() -> Result<(), String> {
    let caller = verified_caller()?;

    match with_state(|s| s.registry.owner_map.is_valid_owner(&caller)) {
        true => Ok(()),
        false => Err("only patient can call this method".to_string()),
    }
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

fn initialize() {
    let state = init_state();
    STATE.replace(Some(state));
    log!("canister state initialized");
    initialize_id_generator();
    ConsentsApi::init();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    initialize()
}

#[ic_cdk::init]
fn canister_init() {
    initialize()
}

#[ic_cdk::query(composite = true, guard = "only_patient")]
async fn read_emr_by_id(req: ReadEmrByIdRequest) -> ReadEmrByIdResponse {
    let user = verified_caller().unwrap();
    let args = with_state(|s| s.registry.construct_args_read_emr(req, &user)).unwrap();

    PatientRegistry::do_call_read_emr(args).await
}

fn emr_list_patient(req: EmrListPatientRequest) -> EmrListPatientResponse {
    let caller = verified_caller().unwrap();
    let nik = with_state(|s| s.registry.owner_map.get_nik(&caller).unwrap()).into_inner();

    with_state(move |s| s.registry.emr_binding_map.emr_list(&nik, req.page, req.limit))
        .unwrap()
        .into()
}

#[ic_cdk::update]
fn notify_issued(req: IssueRequest) {
    with_state_mut(|s|
        s.registry.emr_binding_map.issue_for(req.header.user_id.clone(), req.header)
    ).unwrap();
}

fn authorized_canisters() {
    todo!()
}

#[ic_cdk::update]
fn register_patient(req: RegisterPatientRequest) {
    let owner = verified_caller().unwrap();
    with_state_mut(|s| s.registry.owner_map.bind(owner, req.nik)).unwrap()
}

#[ic_cdk::query(composite = true)]
async fn ping() -> PingResult {
    let emr_registry_status = emr_registry::emr_registry.ping().await.is_ok();

    PingResult {
        emr_registry_status,
    }
}

#[ic_cdk::query]
fn metrics() -> String {
    with_state(|s| {
        [
            opaque_metrics!(s.registry),
            OpaqueMetrics::measure(&**s.config.get()),
            statistics::canister::BlockchainMetrics::measure(),
            statistics::canister::MemoryStatistics::measure(),
        ].join("\n")
    })
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
    ConsentsApi::read_emr_with_session(&req.session_id, req.args).await.unwrap()
}

#[ic_cdk::query]
async fn emr_list_with_session(req: EmrListConsentRequest) -> EmrListConsentResponse {
    ConsentsApi::emr_list_with_session(&req.session_id).unwrap().into()
}

#[ic_cdk::update]
/// Derive the encryption key with the session id securely transported by encrypting the decryption key, used to decrypt emr
async fn derive_encryption_key_with_session(
    req: DeriveSecretKeyRequest
) -> DeriveSecretKeyResponse {
    let consent = ConsentsApi::resolve_session(&req.session_id).expect("session not found");
    vetkd::EncryptionApi::encrypted_emr_decryption_key(req.transport_key, &consent.nik).await.into()
}

#[ic_cdk::update]
/// Derive the encryption verification key with the session id, used to verify the encrypted emr decryption key
async fn derive_encryption_verification_key_with_session(
    req: DeriveVerificationKeyRequest
) -> DeriveVerificationKeyResponse {
    let consent = ConsentsApi::resolve_session(&req.session_id).expect("session not found");
    vetkd::EncryptionApi::verification_key_for(&consent.nik).await.into()
}

#[ic_cdk::update]
fn revoke_consent(req: RevokeConsentRequest) {
    ConsentsApi::revoke_consent(&req.code);
}

#[ic_cdk::update]
fn finish_session(req: FinishSessionRequest) {
    ConsentsApi::finish_sesion(&req.session_id)
}

#[ic_cdk::update]
fn claim_consent(req: ClaimConsentRequest) -> ClaimConsentResponse {
    ConsentsApi::claim_consent(&req.code)
        .expect("consent already claimed or does not exists")
        .into()
}

ic_cdk::export_candid!();
