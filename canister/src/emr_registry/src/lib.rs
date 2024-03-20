use canister_common::{
    common::{ self, RawEmr },
    id_generator::IdGenerator,
    mmgr::MemoryManager,
    random::CanisterRandomSource,
};
use ic_cdk::{ init, query };
use std::cell::RefCell;
use core::time::Duration;

mod key;
mod registry;
mod api;
mod header;

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
        let memory_manager = MemoryManager::new();
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
fn read_emr_by_id(req: api::ReadEmrByIdRequest) -> api::ReadEmrByIdResponse {
    with_state(|s| { s.registry.read_by_id(req.to_read_key()).unwrap().into() })
}

#[query]
fn dummy() {}

ic_cdk::export_candid!();
