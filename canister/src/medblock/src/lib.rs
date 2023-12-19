use std::cell::RefCell;

use candid::Principal;
use config::CanisterConfig;
use emr::{ providers::ProviderRegistry, EmrRegistry, EmrDisplay, FromStableRef };


mod config;
mod emr;
mod encryption;
mod log;
mod macros;
mod types;

#[derive(Default)]
pub struct State {
    emr_registry: EmrRegistry,
    provider_registry: ProviderRegistry,
    config: CanisterConfig,
    // TODO : incorporate logs
    // log: Log,
}

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::default();
}

// guard function
fn only_canister_owner() -> Result<(), String> {
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let caller = ic_cdk::caller();

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

        let caller = ic_cdk::caller();

        if !state.provider_registry.is_valid_provider(&caller) {
            return Err("only provider can call this method".to_string());
        }

        Ok(())
    })
}

// guard function
fn only_patients() -> Result<(), String> {
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let caller = ic_cdk::caller();

        if !state.emr_registry.is_valid_patient(&caller) {
            return Err("only patient can call this method".to_string());
        }

        Ok(())
    })
}

// guard function
fn only_patients_or_provider() -> Result<(), String> {
    only_patients().or_else(|_| only_provider())
}

#[ic_cdk::init]
fn init() {
    ic_stable_memory::stable_memory_init();

    STATE.with(|state| {
        *state.borrow_mut() = Some(State::default());
    });
}

#[ic_cdk::update(guard = "only_canister_owner")]
// TODO : move arguments to a candid struct
fn register_new_provider(new_provider: Principal, encryted_display_name: String) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().unwrap();

        state.provider_registry.register_new_provider(new_provider, encryted_display_name).unwrap()
    });
}

#[ic_cdk::update(guard = "only_canister_owner")]
// TODO : move arguments to a candid struct
fn suspend_provider(provider: Principal) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().unwrap();

        state.provider_registry.suspend_provider(provider).unwrap()
    });
}

#[ic_cdk::query(guard = "only_patients_or_provider")]
// TODO : move arguments to a candid struct
fn read_emr_by_id(emr_id: types::Id) -> Option<emr::EmrDisplay> {
    STATE.with(|state| {
        let state = state.borrow();
        let state = state.as_ref().unwrap();

        let emr = state.emr_registry.get_emr(&emr_id).unwrap();

        Some(EmrDisplay::from_stable_ref(&*emr))
    })
}

ic_cdk::export::candid::export_service!();
