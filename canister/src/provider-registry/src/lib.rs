use std::{ borrow::Borrow, cell::RefCell, rc::Rc, time::Duration };

use canister_common::{
    common::{ self, freeze::FreezeThreshold },
    id_generator::IdGenerator,
    mmgr::MemoryManager,
    random::CanisterRandomSource,
    statistics::{ self, traits::OpaqueMetrics },
};
use ic_principal::Principal;
use registry::ProviderRegistry;

mod registry;
mod config;
mod types;

/// initial seed for the random source, this is insecure and only used for bootstraping the random source before it is reseeded again
/// using true random bytes from ic.
const INSECURE_INITIAL_SEED: u128 = 724361971;

/// TODO: benchmark this
const CANISTER_CYCLE_THRESHOLD: u128 = 300_000;

pub struct State {
    providers: ProviderRegistry,
    config: config::CanisterConfig,
    memory_manager: MemoryManager,
    freeze_threshold: FreezeThreshold,
}

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

#[ic_cdk::inspect_message]
fn inspect_message() {
    verified_caller().expect("caller is not verified");
    with_state(|s| s.freeze_threshold.check());

    ic_cdk::api::call::accept_message()
}

fn verified_caller() -> Result<Principal, String> {
    let caller = ic_cdk::caller();

    ic_cdk::eprintln!("caller : {}", caller);

    if caller.ne(&Principal::anonymous()) {
        return Err(String::from("anonymous caller is not allowed"));
    }
    Ok(caller)
}

// guard function
fn only_canister_owner() -> Result<(), String> {
    return Ok(());
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let caller = verified_caller()?;

        if !state.config.is_canister_owner(&caller) {
            return Err("only canister owner can call this method".to_string());
        }

        Ok(())
    })
}

// guard function
fn only_provider() -> Result<(), String> {
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let caller = verified_caller()?;

        if !state.providers.is_valid_provider(&caller) {
            return Err("only provider can call this method".to_string());
        }

        // safe to unwrap as we already check if caller is a valid provider or not
        if state.providers.is_provider_suspended(&caller).unwrap() {
            return Err("provider is suspended".to_string());
        }

        Ok(())
    })
}

async fn initialize_id_generator() {
    let rng = CanisterRandomSource::new().await;
    let id_generator = IdGenerator::new(rng);

    ID_GENERATOR.with(|id_gen| {
        *id_gen.borrow_mut() = Some(id_generator);
    });

    ic_cdk::print("id generator initialized");
}

#[ic_cdk::init]
fn canister_init() {
    STATE.with(move |state| {
        let memory_manager = MemoryManager::new();

        let init = State {
            providers: ProviderRegistry::new(&memory_manager).into(),
            config: config::CanisterConfig::default(),
            memory_manager,
            freeze_threshold: FreezeThreshold::new(CANISTER_CYCLE_THRESHOLD),
        };

        *state.borrow_mut() = Some(init);

        ic_cdk::print("state initialized");
    });

    ic_cdk_timers::set_timer(Duration::from_secs(3), || ic_cdk::spawn(initialize_id_generator()));
}

#[ic_cdk::query]
fn metrics() -> String {
    with_state(|s| {
        [
            s.providers.measure(),
            statistics::canister::BlockchainMetrics::measure(),
            statistics::canister::MemoryStatistics::measure(),
        ].join("\n")
    })
}

// TODO
// #[ic_cdk::update(guard = "only_provider")]
// #[candid::candid_method(update)]
// // TODO : move arguments to a candid struct
// async fn create_emr_for_user(req: CreateEmrForUserRequest) {
//     ic_cdk::eprintln!("create_emr_for_user: {}", req.emr_records.0);

//     let records = Records::try_from(req.emr_records).unwrap();
//     let id = generate_id().await.unwrap();

//     STATE.with(|state| {
//         let mut state = state.borrow_mut();
//         let state = state.as_mut().unwrap();

//         // change the emr version if upgrade happens
//         let emr = emr::V001::new(id, records).into();

//         let emr_id = state.emr_registry.register_emr(emr, req.owner).unwrap();

//         let caller = verified_caller().unwrap();

//         // increment session
//         let _ =state.provider_registry.issue_emr(&caller, emr_id);
//     })
// }

#[ic_cdk::query(guard = "only_provider")]
// TODO : fix anchor
// TODO : move arguments to a candid struct
fn emr_list_provider(req: types::EmrListProviderRequest) -> types::EmrListProviderResponse {
    with_state(|state| {
        let provider = verified_caller().unwrap();

        let limit = state.config.max_item_per_response().min(req.limit);

        state.providers.get_issued(&provider, req.page, req.limit as u64).unwrap().into()
    })
}

ic_cdk::export_candid!();
