#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

root=$(git rev-parse --show-toplevel)
cd $root/canister

# Check if dfx is running
if ! dfx ping >/dev/null 2>&1; then
    echo -e "${RED}[ERROR]${NC} DFX is not running. Please ensure local.sh is running first."
    exit 1
fi

log_header() {
    echo -e "\n${BLUE}=== $1 ===${NC}\n"
}

log_header "Building canisters"
bash $root/canister/build.sh emr_registry
bash $root/canister/build.sh patient_registry
bash $root/canister/build.sh provider_registry

log_header "Upgrading canisters"
# upgrade with state preservation
dfx canister install emr_registry --network local --mode upgrade --wasm $root/canister/target/wasm32-unknown-unknown/release/emr_registry.wasm -y
dfx canister install patient_registry --network local --mode upgrade --wasm $root/canister/target/wasm32-unknown-unknown/release/patient_registry.wasm -y
dfx canister install provider_registry --network local --mode upgrade --wasm $root/canister/target/wasm32-unknown-unknown/release/provider_registry.wasm -y

# get canister ids for verification
emr_registry_id=$(dfx canister id emr_registry --network local)
patient_registry_id=$(dfx canister id patient_registry --network local)
provider_registry_id=$(dfx canister id provider_registry --network local)

# get public IP for URLs
PUBLIC_IP=$(curl -s ifconfig.me)

# generate URLs for verification
candid_ui=$(dfx canister id __Candid_UI)
emr_registry_ui="http://$PUBLIC_IP:8000/?canisterId=$candid_ui&id=$emr_registry_id"
provider_registry_ui="http://$PUBLIC_IP:8000/?canisterId=$candid_ui&id=$provider_registry_id"
patient_registry_ui="http://$PUBLIC_IP:8000/?canisterId=$candid_ui&id=$patient_registry_id"

echo -e "\n${BLUE}[UPGRADE COMPLETE]${NC}"
echo -e "Verify canisters at:"
echo -e "EMR Registry: ${GREEN}$emr_registry_ui${NC}"
echo -e "Provider Registry: ${GREEN}$provider_registry_ui${NC}"
echo -e "Patient Registry: ${GREEN}$patient_registry_ui${NC}\n" 