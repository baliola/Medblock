#!/bin/bash

# colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# helper functions for logging
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

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# check if openssl is installed
if ! command -v openssl &>/dev/null; then
    log_error "openssl is required but not installed"
    exit 1
fi

# check if all required arguments are provided
if [ "$#" -ne 2 ]; then
    log_error "Invalid number of arguments"
    echo -e "${YELLOW}Usage: $0 <principal> <network>${NC}"
    echo -e "Example: $0 \"principal_id_here\" \"local\""
    exit 1
fi

PRINCIPAL=$1
NETWORK=$2

# validate network parameter
if [ "$NETWORK" != "ic" ] && [ "$NETWORK" != "local" ]; then
    log_error "Network must be either 'ic' or 'local'"
    exit 1
fi

# get the canister ID
CANISTER_ID=$(dfx canister --network "$NETWORK" id patient_registry)
if [ -z "$CANISTER_ID" ]; then
    log_error "Cannot find canister id for patient_registry. Please ensure the canister is deployed."
    exit 1
fi

log_process "Adding admin with the following details:"
echo -e "Canister ID: ${MAGENTA}$CANISTER_ID${NC}"
echo -e "Principal: ${MAGENTA}$PRINCIPAL${NC}"
echo -e "Network: ${MAGENTA}$NETWORK${NC}"
echo # empty line for better readability

log_info "Executing canister call..."
dfx canister --network "$NETWORK" call patient_registry bind_admin_principal_only "(principal \"$PRINCIPAL\")"

if [ $? -eq 0 ]; then
    log_success "Admin addition completed successfully ✨"
else
    log_error "Failed to add admin"
    exit 1
fi
