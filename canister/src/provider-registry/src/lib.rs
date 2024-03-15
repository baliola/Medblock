use std::{ cell::RefCell, rc::Rc };

use canister_common::{
    common::{ freeze::FreezeThreshold, traits::{ ScheduledTask } },
    mmgr::MemoryManager,
    random::CanisterRandomSource,
};
use ic_principal::Principal;
use registry::ProviderRegistry;

mod registry;
mod config;

/// approximate estimation of how much cycles it takes to handle 500 id generation in 1 seconds sustained until exactly 1 minutes before it exhausted.
///
/// 1 id generation cost : 10 random bytes
///
/// 500 id generation cost : 500 x 10 = 5000
///
/// 500 id generation cost in 1 minutes = 500 x 10 x 60 = 300_000
///
/// depending of the time it takes for the random bytes to arrive (~2.5s) and the interval configured, this will fill up naturally over a course of ~6-10h.
const RANDOM_BYTES_THRESHOLD: u64 = 300_000;

/// TODO: benchmark this, for now, the same as random bytes threshold
const CANISTER_CYCLE_THRESHOLD: u128 = 300_000;

pub struct State {
    providers: Rc<ProviderRegistry>,
    rng: Rc<CanisterRandomSource>,
    config: config::CanisterConfig,
    memory_manager: MemoryManager,
    freeze_threshold: Rc<FreezeThreshold>,
}

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::new(None);
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
    with_state(|s| s.freeze_threshold.check());
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

// #[ic_cdk::init]
fn init() {
    STATE.with(|state| {
        let memory_manager = MemoryManager::new();

        let init = State {
            providers: ProviderRegistry::new(&memory_manager).into(),
            rng: Rc::new(CanisterRandomSource::new(RANDOM_BYTES_THRESHOLD)),
            config: config::CanisterConfig::default(),
            memory_manager,
            freeze_threshold: FreezeThreshold::new(CANISTER_CYCLE_THRESHOLD).into(),
        };

        ScheduledTask::start_periodic_task(init.freeze_threshold.clone());
        ScheduledTask::start_periodic_task(init.providers.clone());
        
        *state.borrow_mut() = Some(init);
    });
}
