use ic_cdk_bindgen::{ Builder, Config };
use std::{ panic::catch_unwind, path::PathBuf };

fn get_workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("Cannot find manifest dir")
    );

    for anc in manifest_dir.ancestors() {
        if anc.file_name().unwrap() == "canister" {
            return anc.to_path_buf();
        }
    }

    unreachable!("Cannot find workspace manifest")
}

// workaround for setting provider registry candid path because we run into circular issues.
fn hardcode_set_registry_candid_path() {
    std::env::set_var("CANISTER_CANDID_PATH_PROVIDER_REGISTRY", "src/provider_registry/candid.did");
    // workaround, we dont currenly use it
    std::env::set_var("CANISTER_ID_PROVIDER_REGISTRY", "be2us-64aaa-aaaaa-qaabq-cai");

    std::env::set_var("CANISTER_CANDID_PATH_EMR_REGISTRY", "src/emr_registry/candid.did");
    // workaround, we dont currenly use it
    std::env::set_var("CANISTER_ID_EMR_REGISTRY", "be2us-64aaa-aaaaa-qaabq-cai");

    std::env::set_var("CANISTER_CANDID_PATH_PATIENT_REGISTRY", "src/patient_registry/candid.did");
    // workaround, we dont currenly use it
    std::env::set_var("CANISTER_ID_PATIENT_REGISTRY", "be2us-64aaa-aaaaa-qaabq-cai");
}

fn main() {
    println!("cargo:rerun-if-changed=NULL");

    // workaround to determine if this is invoked by dfx as dfx automatically inject this env var
    let candid_path_env = std::env::var("CANISTER_CANDID_PATH_EMR_REGISTRY").is_ok();

    hardcode_set_registry_candid_path();

    build_declaration();
}
fn get_config(canister: &str) -> Config {
    let mut config = Config::new(canister);

    let path = format!("src/{canister}/candid.did");
    config.candid_path = get_workspace_root().join(path);

    config
}

fn build_declaration() {
    let configs = [
        get_config("emr_registry"),
        get_config("patient_registry"),
        get_config("provider_registry"),
    ];

    let mut builder = Builder::new();

    for config in configs {
        builder.add(config);
    }
    let out = get_workspace_root().join("tests/integration-tests/src/declarations");
    builder.build(Some(out.clone()));
    
    pocket_ic_bindgen::Builder::build(out.clone())
}
