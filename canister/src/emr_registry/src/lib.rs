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
    common,
    id_generator::IdGenerator,
    log,
    mmgr::MemoryManager,
    opaque_metrics,
    random::CanisterRandomSource,
    register_log,
    statistics,
};
use ic_cdk::{ init };
use std::cell::RefCell;
use core::time::Duration;

mod key;
mod registry;
pub mod api;
pub mod header;
mod memory;

type State = common::State<registry::CoreEmrRegistry, (), ()>;
register_log!("emr");

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

// TODO : add init method

fn initialize_id_generator() {
    ic_cdk_timers::set_timer(Duration::from_secs(3), || {
        ic_cdk::spawn(async move {
            let rng = CanisterRandomSource::new().await;
            let id_generator = IdGenerator::new(rng);

            ID_GENERATOR.with(|id_gen| {
                *id_gen.borrow_mut() = Some(id_generator);
            });

            log!("id generator initialized");
        })
    });
}
fn init_state() -> self::State {
    let memory_manager = MemoryManager::init();
    let state = State::new(
        registry::CoreEmrRegistry::init(&memory_manager),
        (),
        (),
        memory_manager
    );

    state
}

fn initialize() {
    let state = init_state();
    STATE.replace(Some(state));
    log!("state initialized");
    initialize_id_generator()
}

#[init]
fn init() {
    initialize();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    initialize();
}

// TODO : add init state
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
fn ping() {
    // no-op
}

#[ic_cdk::query]
fn metrics() -> String {
    with_state(|s| {
        [
            opaque_metrics!(s.registry),
            statistics::canister::BlockchainMetrics::measure(),
            statistics::canister::MemoryStatistics::measure(),
        ].join("\n")
    })
}

ic_cdk::export_candid!();
