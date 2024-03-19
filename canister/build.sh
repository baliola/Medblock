#!/usr/bin/env bash
set -e

root=$(git rev-parse --show-toplevel)/canister
canister=$1
emr_registry_did_path=$root/src/emr_registry/candid.did
canister_did_path=$root/src/$canister/candid.did
wasm_dir=$root/target/wasm32-unknown-unknown/release
canister_wasm=$wasm_dir/$canister.wasm
dummy_did="service :{} "
# build emr registry

cd $root
echo inserting placeholder candid
echo "$dummy_did" >$emr_registry_did_path
echo "Building emr registry"
dfx build emr_registry >/dev/null 2>&1

echo done

# extract candid to emr registry

echo "Extracting emr registry candid"
candid-extractor $wasm_dir/emr_registry.wasm >$emr_registry_did_path
echo done

echo inserting placeholder candid
echo "$dummy_did" >$canister_did_path
echo building $canister canister
dfx build $canister >/dev/null 2>&1
echo done

echo extracting $canister candid from wasm
candid-extractor $canister_wasm >$canister_did_path 
echo done
