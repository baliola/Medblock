#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

root=$(git rev-parse --show-toplevel)

cd $root/canister

bash $root/canister/setup.sh
# This script deploys the canister locally.
FE_PORT=4943
lsof -i tcp:${FE_PORT} | awk 'NR!=1 {print $2}' | xargs kill || true

# Check if --background flag is passed
if [[ "$1" == "--background" ]]; then
  echo -e "${BLUE}[INFO]${NC} Starting dfx in background mode..."
  dfx start --background --host 0.0.0.0:4943
else
  echo -e "${BLUE}[INFO]${NC} Starting dfx in concurrent mode..."
  # Start dfx in the background but keep output visible
  (dfx start --host 0.0.0.0:4943 2>&1 | sed 's/^/[CANISTER] /') &
  DFX_PID=$!
  # Store the PID so we can terminate it later if needed
  echo $DFX_PID >/tmp/dfx.pid

  # Wait for dfx to initialize
  echo -e "${YELLOW}[WAIT]${NC} Waiting for dfx to start..."
  sleep 5
fi

echo -e "${GREEN}[INFO]${NC} Installing canisters..."
dfx canister install emr_registry --wasm $root/canister/target/wasm32-unknown-unknown/release/emr_registry.wasm --mode=install -y

# Rebuild patient registry for Candid UI compatibility
echo -e "${YELLOW}[NOTE]${NC} Rebuilding patient registry for Candid UI compatibility..."
bash $root/canister/build.sh patient_registry

dfx canister install patient_registry --wasm $root/canister/target/wasm32-unknown-unknown/release/patient_registry.wasm --mode=install -y
dfx canister install provider_registry --wasm $root/canister/target/wasm32-unknown-unknown/release/provider_registry.wasm --mode=install -y

DEFAULT_IDENTITY="2vxsx-fae"
emr_registry_id=$(dfx canister id emr_registry)
patient_registry_id=$(dfx canister id patient_registry)
provider_registry_id=$(dfx canister id provider_registry)
# Binding canisters to each other through principal updates

echo -e "${GREEN}[INFO]${NC} Adding anonymous principal as controllers"
bash $root/canister/scripts/utils/add_controller.sh emr_registry "$DEFAULT_IDENTITY" local
bash $root/canister/scripts/utils/add_metrics_collector.sh emr_registry "$DEFAULT_IDENTITY" local

echo -e "${GREEN}[INFO]${NC} Configuring EMR Registry canister"
dfx canister call --network=local emr_registry add_authorized_caller --type idl "(record {caller=principal \"$provider_registry_id\" })" --candid $root/canister/src/emr_registry/candid.did
dfx canister call --network=local emr_registry add_authorized_caller --type idl "(record {caller=principal \"$patient_registry_id\" })" --candid $root/canister/src/emr_registry/candid.did

echo -e "${GREEN}[INFO]${NC} Configuring Patient Registry canister"
bash $root/canister/scripts/utils/add_controller.sh patient_registry "$DEFAULT_IDENTITY" local
bash $root/canister/scripts/utils/add_metrics_collector.sh patient_registry "$DEFAULT_IDENTITY" local
dfx canister call --network=local patient_registry update_emr_registry_principal --type idl "(record {\"principal\"=principal \"$emr_registry_id\" })" --candid $root/canister/src/patient_registry/candid.did
dfx canister call --network=local patient_registry update_provider_registry_principal --type idl "(record {\"principal\"=principal \"$provider_registry_id\" })" --candid $root/canister/src/patient_registry/candid.did

echo -e "${GREEN}[INFO]${NC} Configuring Provider Registry canister"
bash $root/canister/scripts/utils/add_controller.sh provider_registry "$DEFAULT_IDENTITY" local
bash $root/canister/scripts/utils/add_metrics_collector.sh provider_registry "$DEFAULT_IDENTITY" local
dfx canister call --network=local provider_registry update_emr_registry_principal --type idl "(record {\"principal\"=principal \"$emr_registry_id\" })" --candid $root/canister/src/provider_registry/candid.did
dfx canister call --network=local provider_registry update_patient_registry_principal --type idl "(record {\"principal\"=principal \"$patient_registry_id\" })" --candid $root/canister/src/provider_registry/candid.did

echo -e "${GREEN}[INFO]${NC} Deploying Internet Identity"
dfx deploy internet_identity --network=local

# Generate URLs for testing
candid_ui=$(dfx canister id __Candid_UI)
basepath="http://0.0.0.0:4943/?canisterId=$candid_ui"
emr_registry_ui="$basepath&id=$emr_registry_id"
provider_registry_ui="$basepath&id=$provider_registry_id"
patient_registry_ui="$basepath&id=$patient_registry_id"
NIK="3fe93da886732fd563ba71f136f10dffc6a8955f911b36064b9e01b32f8af709"

echo -e "\n${BLUE}[DEPLOYMENT INFO]${NC}"
echo -e "Default identity: ${GREEN}$DEFAULT_IDENTITY${NC}"
echo -e "Dummy NIK: ${GREEN}$NIK${NC}"
echo -e "\nCanister UIs:"
echo -e "EMR Registry: ${GREEN}$emr_registry_ui${NC}"
echo -e "Provider Registry: ${GREEN}$provider_registry_ui${NC}"
echo -e "Patient Registry: ${GREEN}$patient_registry_ui${NC}\n"
