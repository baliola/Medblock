#! bash
root=$(git rev-parse --show-toplevel)

bash $root/canister/build.sh provider_registry
dfx canister install provider_registry --wasm $root/canister/target/wasm32-unknown-unknown/release/provider_registry.wasm --mode=upgrade -y --network staging
echo done
