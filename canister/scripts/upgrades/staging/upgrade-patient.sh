#! bash
root=$(git rev-parse --show-toplevel)

bash $root/canister/build.sh patient_registry
dfx canister install patient_registry --wasm $root/canister/target/wasm32-unknown-unknown/release/patient_registry.wasm --mode=upgrade -y --network staging
echo done
