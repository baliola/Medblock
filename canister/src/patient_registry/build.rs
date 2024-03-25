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
fn hardcode_set_provider_registry_candid_path() {
    std::env::set_var("CANISTER_CANDID_PATH_PROVIDER_REGISTRY", "src/provider_registry/candid.did");
}

fn main() {
    println!("cargo:rerun-if-changed=NULL");

    let link_flag = std::env::var("LINK").unwrap_or("true".to_string()).parse::<bool>().unwrap();
    // workaround to determine if this is invoked by dfx as dfx automatically inject this env var
    let candid_path_env = std::env::var("CANISTER_CANDID_PATH_EMR_REGISTRY").is_ok();

    // dont run this script in test environment
    if !candid_path_env || !link_flag {
        return;
    }
    let a = std::env::var("CANISTER_CANDID_PATH_EMR_REGISTRY").unwrap();
    println!("candid path :{a}");

    hardcode_set_provider_registry_candid_path();

    let result = catch_unwind(build_declaration);
    match result {
        Ok(_) => (),
        Err(_) =>
            panic!(
                "\nERROR: failed to generate foreign canister binding, are you running tests?\nNOTE: run with `LINK=false` to disable this i.e for test/linting, etc.."
            ),
    }
}
fn get_config(canister: &str) -> Config {
    let mut config = Config::new(canister);

    let path = format!("src/{canister}/candid.did");
    config.candid_path = get_workspace_root().join(path);

    config
}

fn build_declaration() {
    let configs = [get_config("emr_registry"), get_config("provider_registry")];

    let mut builder = Builder::new();

    for config in configs {
        builder.add(config);
    }

    builder.build(Some(get_workspace_root().join("src/patient_registry/src/declarations")));
}
