use std::{ path::Path, process::Command };

use candid::{ CandidType, Encode };
use ic_cdk::api::management_canister::main::CanisterSettings;
use ic_principal::Principal;
use integration_tests::declarations;
use pocket_ic::UserError;

const PROVIDER_REGISTRY_WASM: &[u8] = include_bytes!(
    "../../../../target/wasm32-unknown-unknown/release/provider_registry.wasm"
);
const PATIENT_REGISTRY_WASM: &[u8] = include_bytes!(
    "../../../../target/wasm32-unknown-unknown/release/patient_registry.wasm"
);
const EMR_REGISTRY_WASM: &[u8] = include_bytes!(
    "../../../../target/wasm32-unknown-unknown/release/emr_registry.wasm"
);

const POCKET_IC_PATH: Option<&'static str> = std::option_env!("POCKET_IC_BIN");

// 10 Triliion cycle
const CYCLE: u128 = 10_000_000_000_000;

fn create_canister(
    server: &pocket_ic::PocketIc,
    bytes: &'static [u8],
    arg: Vec<u8>,
    controller: Principal
) -> ic_cdk::api::management_canister::provisional::CanisterId {
    let id = server.create_canister_with_settings(
        Some(controller.clone()),
        Some(CanisterSettings {
            controllers: Some(vec![controller.clone()]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        })
    );

    server.add_cycles(id.clone(), CYCLE);
    server.install_canister(id.clone(), bytes.to_vec(), arg, Some(controller));

    id
}

fn setup_emr_registry(
    server: &pocket_ic::PocketIc,
    controller: Principal
) -> ic_cdk::api::management_canister::provisional::CanisterId {
    let id = create_canister(server, EMR_REGISTRY_WASM, Vec::new(), controller);
    id
}

fn bind_emr_registry(
    server: &pocket_ic::PocketIc,
    id: Principal,
    controller: Principal,
    provider: Principal,
    patient: Principal
) {
    let args = declarations::emr_registry::AuthorizedCallerRequest {
        caller: patient.clone(),
    };

    server.update_call(
        id.clone(),
        controller.clone(),
        "add_authorized_caller",
        Encode!(&args).unwrap()
    );

    let args = declarations::emr_registry::AuthorizedCallerRequest {
        caller: provider.clone(),
    };

    server.update_call(
        id.clone(),
        controller.clone(),
        "add_authorized_caller",
        Encode!(&args).unwrap()
    );
}

fn setup_provider_registry(
    server: &pocket_ic::PocketIc,
    controller: Principal
) -> ic_cdk::api::management_canister::provisional::CanisterId {
    let id = create_canister(server, PROVIDER_REGISTRY_WASM, Vec::new(), controller);
    id
}

fn bind_provider_registry(
    server: &pocket_ic::PocketIc,
    id: Principal,
    controller: Principal,
    emr: Principal,
    patient: Principal
) {
    let args = declarations::provider_registry::AuthorizedCallerRequest {
        caller: emr.clone(),
    };

    // bind the provider registry to the emr registry
    server.update_call(
        id.clone(),
        controller.clone(),
        "update_emr_registry_principal",
        Encode!(&args).unwrap()
    );

    // bind the provider registry to the patient registry
    let args = declarations::provider_registry::AuthorizedCallerRequest {
        caller: patient.clone(),
    };

    server.update_call(
        id.clone(),
        controller.clone(),
        "update_patient_registry_principal",
        Encode!(&args).unwrap()
    );
}

fn setup_patient_registry(
    server: &pocket_ic::PocketIc,
    controller: Principal
) -> ic_cdk::api::management_canister::provisional::CanisterId {
    let id = create_canister(server, PATIENT_REGISTRY_WASM, Vec::new(), controller);

    id
}

fn bind_patient_registry(
    server: &pocket_ic::PocketIc,
    id: Principal,
    controller: Principal,
    emr: Principal,
    provider: Principal
) {
    let args = declarations::patient_registry::AuthorizedCallerRequest {
        caller: emr.clone(),
    };

    // bind the patient registry to the emr registry
    server.update_call(
        id.clone(),
        controller.clone(),
        "update_emr_registry_principal",
        Encode!(&args).unwrap()
    );

    // bind the patient registry to the provider registry
    let args = declarations::patient_registry::AuthorizedCallerRequest {
        caller: provider.clone(),
    };

    server.update_call(
        id.clone(),
        controller.clone(),
        "update_provider_registry_principal",
        Encode!(&args).unwrap()
    );
}

// try to resolve pocket ic binary from path
fn resolve_pocket_ic_path() {
    if POCKET_IC_PATH.is_none() {
        let path = Command::new("which")
            .arg("pocket-ic")
            .output()
            .expect("pocket-ic not found").stdout;

        let path = std::str::from_utf8(&path).unwrap().trim();
        let path = Path::new(path);
        let path = path.to_str().unwrap();

        std::env::set_var("POCKET_IC_BIN", path);
    }
}

fn bind_canisters(
    server: &pocket_ic::PocketIc,
    provider: Principal,
    patient: Principal,
    emr: Principal,
    controller: Principal
) {
    bind_emr_registry(server, emr.clone(), controller.clone(), provider.clone(), patient.clone());
    bind_patient_registry(
        server,
        patient.clone(),
        controller.clone(),
        emr.clone(),
        provider.clone()
    );
    bind_provider_registry(
        server,
        provider.clone(),
        controller.clone(),
        emr.clone(),
        patient.clone()
    );
}

pub struct Registries {
    ic: pocket_ic::PocketIc,
    emr: Principal,
    patient: Principal,
    provider: Principal,
}

pub fn prepare_env() -> Registries {
    resolve_pocket_ic_path();

    let server = pocket_ic::PocketIcBuilder::new().with_application_subnet().build();
    let controller = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();

    let id = Principal::anonymous();

    let provider = setup_provider_registry(&server, controller.clone());
    let patient = setup_patient_registry(&server, controller.clone());
    let emr = setup_emr_registry(&server, controller.clone());

    bind_canisters(&server, provider.clone(), patient.clone(), emr.clone(), controller.clone());

    Registries {
        ic: server,
        emr,
        patient,
        provider,
    }
}
