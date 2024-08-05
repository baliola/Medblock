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
- `ic-wasm` (to install run `cargo install ic-wasm`)

For the first time, run the `setup.sh` script, this will do a few things 
- check installations of required tools
- create canisters id
- generate canister binding
- compile all 3 canisters

note that this process might take a while depending on the specs of your machine.

# Development Workflow
If you are developing a feature for a canister, make sure to use the `build.sh` scripts, as it will automatically regenerate candid interface of the canister you're using, it'll try to recompile `emr registry` canister each time it's been called, this is necesarry as the two other crates depend on the emr registry. If you alreadty run the setup script then this step should not be time consuming. Notably, pass `--all` flag to the script if you want to rebuild all canister simultaneously.

# Deploy Locally
To deploy locally, simply go to `scripts/deployments` and execute `local.sh`
```bash
cd scripts/deployments
./local.sh
```
This will do the following things :
- remove previous instance of dfx 
- run `setup.sh`, so make sure you install the dependencies above.
- deploy the 3 canisters using your current dfx identity
- link all 3 canisters together
- add anonymous caller as authorized caller

# Run tests
To run tests, please compile and run it using the `--release` flag. Otherwise, the test will take too long to complete (~30 minutes in slower computer).

Unit test 
```bash
cd canister
cargo test --release
```

## Integration tests
For integration test, please install [pocket-ic](https://github.com/dfinity/pocketic?tab=readme-ov-file). after you install it, follow the step below

```bash
./build.sh --all
```
> build all canisters, please do so after you make any changes to the canister code. As running the integration test directly won't recompile the canisters.

Compile and run the integration tests

```bash
cd canister/tests/integration-tests
cargo test --release
```


