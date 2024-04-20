#! /usr/bin/bash

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

# if the first args is --all, build all canister
if [ "$canister" == "--all" ]; then
    echo "Building all canisters"
    bash $root/build.sh $emr_canister
    bash $root/build.sh $provider_canister
    bash $root/build.sh $patient_canister
    exit 0
fi

# build emr registry
if [ -z "$canister" ]; then
    echo "Usage: $0 <canister_name> 
    canister_name : [$emr_canister, $provider_canister, $patient_canister]"

    exit 0
fi

cd $root
echo "Building emr registry"
dfx build $emr_canister >/dev/null 2>&1
echo inserting placeholder candid
echo "$dummy_did" >$emr_registry_did_path

echo done

# extract candid to emr registry

echo "Extracting emr registry candid"
candid-extractor $wasm_dir/$emr_canister.wasm >$emr_registry_did_path

ic-wasm "$wasm_dir/$emr_canister.wasm" \
    -o "$wasm_dir/$emr_canister.wasm" \
    metadata candid:service -v public -f $emr_registry_did_path

ic-wasm "$wasm_dir/$emr_canister.wasm" \
    -o "$wasm_dir/$emr_canister.wasm" \
    shrink
echo done

if [ "$canister" == "$emr_canister" ]; then
    exit 0
fi

# build canister

echo building $canister canister
dfx build $canister >/dev/null 2>&1
echo inserting placeholder candid
echo "$dummy_did" >$canister_did_path
echo done

echo extracting $canister candid from wasm
candid-extractor $canister_wasm >$canister_did_path

ic-wasm "$canister_wasm" \
    -o "$canister_wasm" \
    metadata candid:service -v public -f $canister_did_path

ic-wasm "$canister_wasm" \
    -o "$canister_wasm" \
    shrink
echo done

echo "shrinking wasm size"
candid-extractor $wasm_dir/$emr_canister.wasm >$emr_registry_did_path

ic-wasm "$wasm_dir/$emr_canister.wasm" \
    -o "$wasm_dir/$emr_canister.wasm" \
    metadata candid:service -v public -f $emr_registry_did_path

ic-wasm "$wasm_dir/$emr_canister.wasm" \
    -o "$wasm_dir/$emr_canister.wasm" \
    shrink
echo done
