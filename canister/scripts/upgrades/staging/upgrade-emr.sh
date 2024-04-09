#! bash
root=$(git rev-parse --show-toplevel)

bash $root/canister/build.sh emr_registry
dfx canister install emr_registry --wasm $root/canister/target/wasm32-unknown-unknown/release/emr_registry.wasm --mode=upgrade -y --network staging
echo done