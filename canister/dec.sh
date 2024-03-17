#! bash
ROOT=$(git rev-parse --show-toplevel)/canister

echo building provider registry
cargo dfx build provider-registry >/dev/null 2>&1
echo done

echo "generating did definition for provider registry"
dfx generate provider-registry
echo done
