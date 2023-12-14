use std::cell::RefCell;

use emr::{providers::ProviderRegistry, EmrRegistry};
use ic_stable_memory::collections::SLog;

mod emr;
mod encryption;
mod log;
mod macros;
mod types;

#[derive(Default)]
pub struct State {
    emr_registry: EmrRegistry,
    provider_registry: ProviderRegistry,
    // TODO : incorporate logs
    // log: Log,
}

thread_local! {
    static STATE: RefCell<State> = State::default().into();
}

#[ic_cdk::init]
fn init() {
    ic_stable_memory::stable_memory_init();
}
