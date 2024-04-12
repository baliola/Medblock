use std::time::Duration;

use integration_tests::declarations::provider_registry::{
    pocket_ic_bindings::Call,
    AuthorizedCallerRequest,
};

mod common;

#[test]
fn test() {
    let registries: common::Registries = common::prepare_env();

    registries.provider.add_authorized_metrics_collector(
        &registries.ic,
        registries.controller.clone(),
        Call::Update,
        AuthorizedCallerRequest {
            caller: registries.controller.clone(),
        }
    ).unwrap();

    // sleep until everything is initialized
    let result = registries.provider
        .ping(&registries.ic, registries.controller.clone(), Call::Query)
        .unwrap();
}
