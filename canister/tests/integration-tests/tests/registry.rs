use integration_tests::declarations::provider_registry::pocket_ic_bindings::Call;

mod common;

#[test]
fn test() {
    let registries: common::Registries = common::prepare_env();

    let result = registries.provider
        .ping(&registries.ic, registries.controller.clone(), Call::Query)
        .unwrap();
}
