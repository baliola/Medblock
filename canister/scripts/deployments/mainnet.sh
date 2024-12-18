#!/bin/bash

# Get the directory of the current script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." && pwd )"

source "$PROJECT_ROOT/scripts/utils/log.sh"

cd "$PROJECT_ROOT"

log_warning "This will deploy canisters to mainnet. This is a production environment."
echo "Are you sure you want to continue? (y/N)"
read -r response
if [[ ! "$response" =~ ^[Yy]$ ]]; then
    log_info "Operation cancelled"
    exit 1
fi

log_header "Building canisters"
bash $PROJECT_ROOT/build.sh emr_registry
bash $PROJECT_ROOT/build.sh patient_registry
bash $PROJECT_ROOT/build.sh provider_registry
log_success "All canisters built"

log_header "Installing canisters"
dfx canister install emr_registry --network ic --wasm $PROJECT_ROOT/target/wasm32-unknown-unknown/release/emr_registry.wasm --mode=install
dfx canister install patient_registry --network ic --wasm $PROJECT_ROOT/target/wasm32-unknown-unknown/release/patient_registry.wasm --mode=install
dfx canister install provider_registry --network ic --wasm $PROJECT_ROOT/target/wasm32-unknown-unknown/release/provider_registry.wasm --mode=install
log_success "All canisters installed"

log_header "Getting canister IDs"
emr_registry_id=$(dfx canister id emr_registry --network ic)
patient_registry_id=$(dfx canister id patient_registry --network ic)
provider_registry_id=$(dfx canister id provider_registry --network ic)

log_header "Binding canisters"
# add authorized callers to EMR registry
log_info "Adding authorized callers to EMR registry..."
dfx canister call --network ic emr_registry add_authorized_caller --type idl "(record {caller=principal \"$provider_registry_id\" })" --candid $PROJECT_ROOT/src/emr_registry/candid.did
dfx canister call --network ic emr_registry add_authorized_caller --type idl "(record {caller=principal \"$patient_registry_id\" })" --candid $PROJECT_ROOT/src/emr_registry/candid.did

# update patient registry principals
log_info "Updating patient registry principals..."
dfx canister call --network ic patient_registry update_emr_registry_principal --type idl "(record {\"principal\"=principal \"$emr_registry_id\" })" --candid $PROJECT_ROOT/src/patient_registry/candid.did
dfx canister call --network ic patient_registry update_provider_registry_principal --type idl "(record {\"principal\"=principal \"$provider_registry_id\" })" --candid $PROJECT_ROOT/src/patient_registry/candid.did

# update provider registry principals
log_info "Updating provider registry principals..."
dfx canister call --network ic provider_registry update_emr_registry_principal --type idl "(record {\"principal\"=principal \"$emr_registry_id\" })" --candid $PROJECT_ROOT/src/provider_registry/candid.did
dfx canister call --network ic provider_registry update_patient_registry_principal --type idl "(record {\"principal\"=principal \"$patient_registry_id\" })" --candid $PROJECT_ROOT/src/provider_registry/candid.did

log_success "All canisters bound successfully"

log_header "Deployment Summary"
log_info "EMR Registry ID: $emr_registry_id"
log_info "Patient Registry ID: $patient_registry_id"
log_info "Provider Registry ID: $provider_registry_id" 