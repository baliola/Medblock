use std::{cell::RefCell, time::Duration};

use api::ReadEmrByIdRequest;
use canister_common::{
    common::guard::verified_caller, id_generator::IdGenerator, mmgr::MemoryManager, random::CanisterRandomSource
};
use declarations::emr_registry::ReadEmrByIdResponse;
use registry::PatientRegistry;

mod registry;
mod memory;
mod declarations;
mod api;

type State = canister_common::common::State<registry::PatientRegistry, (), ()>;

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::new(None);
    static ID_GENERATOR: RefCell<Option<IdGenerator<CanisterRandomSource>>> = RefCell::new(None);
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
    return Ok(());
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
        config: (),
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

fn emr_list_patient() {
    todo!()
}
fn notify_issued() {
    todo!()
}
fn authorized_canisters() {
    todo!()
}
fn register_patient() {
    todo!()
}

ic_cdk::export_candid!();
