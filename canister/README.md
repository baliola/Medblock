# Medblock

Medblock EMR registry canister, this project is divided into 3 canisters that each serve a distinct purpose

- Patient registry
- Emr registry
- Provider registry

# Setting up the project

Make sure you have the following tools installed

- dfx (refer to [dfinity docs](https://internetcomputer.org/docs/current/developer-docs/getting-started/install/))
- candid-extractor (to install run `cargo install candid-extractor`)
- rust wasm32 target (to install run `rustup target add wasm32-unknown-unknown`)

For the first time, run the `setup.sh` script, this will do a few things 
- check installations of required tools
- create canisters id
- generate canister binding
- compile all 3 canisters

note that this process might take a while depending on the specs of your machine.

# Development Workflow
If you are developing a feature for a canister, make sure to use the `build.sh` scripts, as it will automatically regenerate candid interface of the canister you're using, it'll try to recompile `emr registry` canister each time it's been called, this is necesarry as the two other crates depend on the emr registry. If you alreadty run the setup script then this step should not be time consuming. 