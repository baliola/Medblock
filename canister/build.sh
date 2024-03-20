#!/usr/bin/env bash
set -e

root=$(git rev-parse --show-toplevel)/canister
canister=$1
emr_registry_did_path=$root/src/emr_registry/candid.did
canister_did_path=$root/src/$canister/candid.did
wasm_dir=$root/target/wasm32-unknown-unknown/release
canister_wasm=$wasm_dir/$canister.wasm
dummy_did="service :{} "
emr_canister="emr_registry"
provider_canister="provider_registry"
patient_canister="patient_registry"

# build emr registry
if [ -z "$canister" ]; then
    echo "Usage: $0 <canister_name> 
    canister_name : [$emr_canister, $provider_canister, $patient_canister]"

    exit 0
fi

cd $root
echo inserting placeholder candid
echo "$dummy_did" >$emr_registry_did_path
echo "Building emr registry"
dfx build $emr_canister >/dev/null 2>&1

echo done

# extract candid to emr registry

echo "Extracting emr registry candid"
candid-extractor $wasm_dir/$emr_canister.wasm >$emr_registry_did_path
echo done

if [ "$canister" == "$emr_canister" ]; then
    exit 0
fi

# build canister

echo inserting placeholder candid
echo "$dummy_did" >$canister_did_path
echo building $canister canister
dfx build $canister >/dev/null 2>&1
echo done

echo extracting $canister candid from wasm
candid-extractor $canister_wasm >$canister_did_path
echo done
