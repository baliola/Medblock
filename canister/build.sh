#!/bin/bash

# colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# get root directory and setup variables
root=$(git rev-parse --show-toplevel)/canister
canister=$1

# canister names
emr_canister="emr_registry"
provider_canister="provider_registry"
patient_canister="patient_registry"

# file paths
emr_registry_did_path=$root/src/emr_registry/candid.did
canister_did_path=$root/src/$canister/candid.did
wasm_dir=$root/target/wasm32-unknown-unknown/release
canister_wasm=$wasm_dir/$canister.wasm

# placeholder candid service
dummy_did="service :{} "

# helper function for logging
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_process() {
    echo -e "${CYAN}[PROCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# helper function to build and process a canister
build_canister() {
    local canister_name=$1
    local wasm_path=$wasm_dir/$canister_name.wasm
    local did_path=$root/src/$canister_name/candid.did

    log_process "Building ${MAGENTA}$canister_name${NC} canister..."
    dfx build $canister_name >/dev/null 2>&1

    log_info "Inserting placeholder candid..."
    echo "$dummy_did" >$did_path

    log_process "Extracting candid from wasm..."
    candid-extractor $wasm_path >$did_path

    # add candid metadata and shrink wasm
    log_process "Processing WASM file..."
    ic-wasm "$wasm_path" \
        -o "$wasm_path" \
        metadata candid:service -v public -f $did_path

    ic-wasm "$wasm_path" \
        -o "$wasm_path" \
        shrink

    log_success "Successfully processed ${MAGENTA}$canister_name${NC}"
    echo # empty line for better readability
}

# build all canisters if --all flag is passed
if [ "$canister" == "--all" ]; then
    log_info "Building all canisters..."
    echo # empty line for better readability
    bash $root/build.sh $emr_canister
    bash $root/build.sh $provider_canister
    bash $root/build.sh $patient_canister
    log_success "All canisters built successfully"
    exit 0
fi

# show usage if no canister specified
if [ -z "$canister" ]; then
    echo -e "${YELLOW}Usage: $0 <canister_name>${NC}"
    echo -e "Available canisters: [${MAGENTA}$emr_canister${NC}, ${MAGENTA}$provider_canister${NC}, ${MAGENTA}$patient_canister${NC}]"
    exit 0
fi

cd $root

# build EMR registry first since other canisters depend on it
if [ "$canister" == "$emr_canister" ]; then
    build_canister $emr_canister
    log_success "EMR Registry build completed"
    exit 0
fi

# for other canisters, build EMR registry first then build target canister
log_warning "EMR Registry is a dependency, building it first..."
build_canister $emr_canister

log_info "Building target canister..."
build_canister $canister

log_success "Build process completed successfully ✨"
