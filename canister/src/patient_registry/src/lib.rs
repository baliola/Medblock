use std::{ cell::RefCell, time::Duration };

use api::{
    EmrListPatientRequest, EmrListPatientResponse, PingResult, ReadEmrByIdRequest, RegisterPatientRequest
};
use canister_common::{
    common::guard::verified_caller,
    id_generator::IdGenerator,
    mmgr::MemoryManager,
    random::CanisterRandomSource,
    stable::{ Candid, Memory, Stable },
};
use config::CanisterConfig;
use declarations::{emr_registry::{ self, ReadEmrByIdResponse }, provider_registry::provider_registry};
use ic_stable_structures::Cell;
use registry::PatientRegistry;

mod registry;
mod memory;
mod declarations;
mod api;
mod config;

type State = canister_common::common::State<
    registry::PatientRegistry,
    Cell<Stable<CanisterConfig, Candid>, Memory>,
    ()
>;

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

            ic_cdk::print("id generator initialized");
        })
    });
}

fn initialize() {
    let state = init_state();
    STATE.replace(Some(state));
    ic_cdk::print("canister state initialized");
    initialize_id_generator()
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    initialize()
}

#[ic_cdk::init]
fn canister_init() {
    initialize()
}

#[ic_cdk::query(composite = true)]
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

fn notify_issued() {
    todo!()
}

fn authorized_canisters() {
    todo!()
}

fn register_patient(req: RegisterPatientRequest) {
    let owner = verified_caller().unwrap();
    with_state_mut(|s| s.registry.owner_map.bind(owner, req.nik)).unwrap()
}

async fn ping() -> PingResult {
    let emr_registry_status = emr_registry::emr_registry.ping().await.is_ok();

    // let provider_registry_status = provider_registry

    PingResult {
        emr_registry_status,
        patient_registry_status: false,
    }
}

ic_cdk::export_candid!();
