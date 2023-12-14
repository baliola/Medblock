use std::cell::RefCell;

use config::CanisterConfig;
use emr::{providers::ProviderRegistry, EmrRegistry};
use ic_stable_memory::collections::SLog;

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

#[ic_cdk::init]
fn init() {
    ic_stable_memory::stable_memory_init();

    STATE.with(|state| {
        *state.borrow_mut() = Some(State::default());
    });
}
