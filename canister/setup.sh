#!/bin/bash

ROOT=$(git rev-parse --show-toplevel)/canister

# change to canister directory for dfx commands (cross-platform compatible)
cd $ROOT

echo "checking required dependencies"

if ! command -v dfx &>/dev/null; then
    echo "dfx could not be found, please install it"
    exit
fi

if ! command -v ic-wasm &>/dev/null; then
    echo "ic-wasm could not be found, please install it"
    exit
fi

if ! command -v candid-extractor &>/dev/null; then
    echo "candid-extractor could not be found, installing.."
    cargo install candid-extractor
fi

echo "starting ic replica in the background"
dfx stop >/dev/null 2>&1
dfx start --background --clean

echo "creating canisters"
dfx canister create provider_registry
dfx canister create patient_registry
dfx canister create emr_registry

echo "building canisters for the first time"
echo "note: build order: emr_registry -> patient_registry -> provider_registry"

echo "building emr registry"
bash $ROOT/build.sh emr_registry || exit 1

echo "building patient registry"
bash $ROOT/build.sh patient_registry || exit 1

echo "building provider registry"
bash $ROOT/build.sh provider_registry || exit 1

echo "stopping ic replica"
dfx stop
