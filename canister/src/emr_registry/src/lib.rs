use api::{
    CreateEmrRequest,
    CreateEmrResponse,
    ReadEmrByIdRequest,
    ReadEmrByIdResponse,
    RemoveEmrRequest,
    RemoveEmrResponse,
    UpdateEmrRequest,
    UpdateEmrResponse,
};
use canister_common::{
    common::{ self },
    id_generator::IdGenerator,
    mmgr::MemoryManager,
    random::CanisterRandomSource,
};
use ic_cdk::{ init };
use std::cell::RefCell;
use core::time::Duration;

mod key;
mod registry;
pub mod api;
mod header;
mod memory;

type State = common::State<registry::CoreEmrRegistry, (), ()>;

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

fn initialize_id_generator() {
    ic_cdk_timers::set_timer(Duration::from_secs(3), || {
        ic_cdk::spawn(async move {
            let rng = CanisterRandomSource::new().await;
            let id_generator = IdGenerator::new(rng);

            ID_GENERATOR.with(|id_gen| {
                *id_gen.borrow_mut() = Some(id_generator);
            });

            ic_cdk::print("id generator initialized");
        })
    });
}

#[init]
fn init() {
    STATE.with(|s| {
        let memory_manager = MemoryManager::init();
        let state = State::new(
            registry::CoreEmrRegistry::new(&memory_manager),
            (),
            (),
            memory_manager
        );

        s.replace(Some(state));

        ic_cdk::print("state initialized");
    });

    initialize_id_generator();
}

#[ic_cdk::query]
fn read_emr_by_id(req: ReadEmrByIdRequest) -> ReadEmrByIdResponse {
    with_state(|s| { s.registry.read_by_id(req.to_read_key()).unwrap().into() })
}

#[ic_cdk::update]
fn create_emr(req: CreateEmrRequest) -> CreateEmrResponse {
    let id = with_id_generator_mut(|id_gen| id_gen.generate_id());
    let (key, emr) = req.to_args(id);

    with_state_mut(|s| s.registry.add(key, emr))
        .unwrap()
        .into()
}

#[ic_cdk::update]
fn update_emr(req: UpdateEmrRequest) -> UpdateEmrResponse {
    with_state_mut(|s|
        s.registry.update_batch(req.header.to_partial_update_key(), req.fields).unwrap().into()
    )
}

#[ic_cdk::update]
fn remove_emr(req: RemoveEmrRequest) -> RemoveEmrResponse {
    with_state_mut(|s|
        s.registry
            .remove_record(req.header.to_emr_key())
            .map(|_| RemoveEmrResponse::new(true))
            .unwrap()
    )
}

// this will serve as an synchronization function in the future, for now it's only for testing inter-canister calls successfully
#[ic_cdk::query]
fn ping(){
    // no-op
}


ic_cdk::export_candid!();
