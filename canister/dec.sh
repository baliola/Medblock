#! bash
ROOT=$(git rev-parse --show-toplevel)/canister

echo building provider registry
cargo dfx build provider-registry >/dev/null 2>&1
echo done

echo "generating did definition for provider registry"
candid-extractor $ROOT/target/wasm32-unknown-unknown/release/provider_registry.wasm >$ROOT/src/provider-registry/provider-registry.did
echo done
