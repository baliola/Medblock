use ic_cdk_bindgen::{ Builder, Config };
use std::{ fs, path::PathBuf, process::{ Command, Stdio } };

fn get_workspace_root() -> PathBuf {
    let mut manifest_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("Cannot find manifest dir")
    );

    for anc in manifest_dir.ancestors() {
        if anc.file_name().unwrap() == "canister" {
            return anc.to_path_buf();
        }
    }

    unreachable!("Cannot find workspace manifest")
}

fn main() {
    // A workaround to force always rerun build.rs
    println!("cargo:rerun-if-changed=NULL");
    let manifest_dir = get_workspace_root();

    let mut emr = Config::new("emr_registry");

    emr.candid_path = manifest_dir.join("src/emr_registry/candid.did");

    let workspace_cargo_toml_manifest_path = manifest_dir.join("Cargo.toml");

    let wasm_path = manifest_dir.join(
        "target/wasm32-unknown-unknown/release/provider_registry.wasm"
    );

    let mut builder = Builder::new();
    builder.add(emr);
    builder.build(Some(manifest_dir.join("src/provider_registry/src/declarations")));
}
