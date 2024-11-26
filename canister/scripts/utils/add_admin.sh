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

# check if all required arguments are provided
if [ "$#" -ne 3 ]; then
    log_error "Invalid number of arguments"
    echo -e "${YELLOW}Usage: $0 <nik> <principal> <network>${NC}"
    echo -e "Example: $0 1234567890 \"principal_id_here\" \"ic\""
    exit 1
fi

NIK=$1
PRINCIPAL=$2
NETWORK=$3

# validate network parameter
if [ "$NETWORK" != "ic" ] && [ "$NETWORK" != "local" ]; then
    log_error "Network must be either 'ic' or 'local'"
    exit 1
fi

# set the canister name based on your project
CANISTER_NAME="patient_registry"

log_process "Adding admin with the following details:"
echo -e "NIK: ${MAGENTA}$NIK${NC}"
echo -e "Principal: ${MAGENTA}$PRINCIPAL${NC}"
echo -e "Network: ${MAGENTA}$NETWORK${NC}"
echo # empty line for better readability

log_info "Executing canister call..."
dfx canister --network "$NETWORK" call "$CANISTER_NAME" bind_admin "(record { nik=\"$NIK\"; principal=$PRINCIPAL })"

log_success "Admin addition completed successfully ✨" 