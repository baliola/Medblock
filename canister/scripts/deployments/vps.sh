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

# Kill any existing dfx processes first
echo -e "${BLUE}[INFO]${NC} Cleaning up existing processes..."
pkill dfx || true
pkill pocket-ic || true
pkill replica || true
pkill ic-https-outcalls-adapter || true

# Force kill anything on our ports
fuser -k 8000/tcp || true
fuser -k 4943/tcp || true

# Clean up dfx state
rm -rf .dfx/local || true
rm -rf /root/.cache/dfinity/versions/*/replica-configuration || true
rm -rf /root/.config/dfx/replica-configuration || true

# Wait for ports to be fully released
sleep 5

# Verify ports are free
if lsof -i :8000 || lsof -i :4943; then
    echo -e "${RED}[ERROR]${NC} Ports still in use after cleanup"
    exit 1
fi

# Create dfx.json network config
echo -e "${BLUE}[INFO]${NC} Configuring dfx network..."
cat >dfx.json <<EOF
{
  "canisters": {
    "provider_registry": {
      "candid": "src/provider_registry/candid.did",
      "package": "provider_registry",
      "type": "rust",
      "dependencies": ["emr_registry", "patient_registry"]
    },
    "emr_registry": {
      "candid": "src/emr_registry/candid.did",
      "package": "emr_registry",
      "type": "rust"
    },
    "patient_registry": {
      "candid": "src/patient_registry/candid.did",
      "package": "patient_registry",
      "type": "rust",
      "dependencies": ["emr_registry"]
    },
    "internet_identity": {
      "type": "custom",
      "candid": "https://github.com/dfinity/internet-identity/releases/latest/download/internet_identity.did",
      "wasm": "https://github.com/dfinity/internet-identity/releases/latest/download/internet_identity_dev.wasm.gz",
      "remote": {
        "id": {
          "ic": "rdmx6-jaaaa-aaaaa-aaadq-cai"
        }
      },
      "frontend": {}
    }
  },
  "defaults": {
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "networks": {
    "local": {
      "bind": "0.0.0.0:8000",
      "type": "ephemeral",
      "replica": {
        "subnet_type": "system"
      }
    }
  },
  "version": 1
}
EOF

# Start dfx with explicit binding to all interfaces
echo -e "${BLUE}[INFO]${NC} Starting dfx..."
DFX_BIND_ADDRESS=0.0.0.0:8000 dfx start --clean --host 0.0.0.0:8000 --background

# Wait for dfx to initialize
echo -e "${YELLOW}[WAIT]${NC} Waiting for dfx to start..."
sleep 10

# Create canisters with specific IDs
echo -e "${BLUE}[INFO]${NC} Creating canisters..."
dfx canister create --all

# Verify dfx is running
echo -e "${BLUE}[INFO]${NC} Verifying dfx status..."
dfx ping || {
    echo -e "${RED}[ERROR]${NC} DFX not running"
    exit 1
}

# After starting dfx, add this debug section
echo -e "${BLUE}[INFO]${NC} Debug information:"
echo "1. Process list:"
ps aux | grep dfx
echo -e "\n2. Network connections:"
netstat -tulpn | grep -E ':8000|:4943'
echo -e "\n3. DFX status:"
dfx status
echo -e "\n4. Testing local connections:"
curl -v http://localhost:8000/api/v2/status
curl -v http://localhost:4943/api/v2/status

# Verify bindings
echo -e "${BLUE}[INFO]${NC} Verifying network bindings..."
netstat -tulpn | grep -E ':8000|:4943'

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

# Rest of the configuration remains the same as local.sh
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

# Get public IP for URLs
PUBLIC_IP=$(curl -s ifconfig.me)

# Generate URLs for testing
candid_ui=$(dfx canister id __Candid_UI)
basepath="http://$PUBLIC_IP:8000/?canisterId=$candid_ui"
emr_registry_ui="http://$PUBLIC_IP:8000/?canisterId=$candid_ui&id=$emr_registry_id"
provider_registry_ui="http://$PUBLIC_IP:8000/?canisterId=$candid_ui&id=$provider_registry_id"
patient_registry_ui="http://$PUBLIC_IP:8000/?canisterId=$candid_ui&id=$patient_registry_id"
NIK="3fe93da886732fd563ba71f136f10dffc6a8955f911b36064b9e01b32f8af709"

echo -e "\n${BLUE}[DEPLOYMENT INFO]${NC}"
echo -e "Default identity: ${GREEN}$DEFAULT_IDENTITY${NC}"
echo -e "Dummy NIK: ${GREEN}$NIK${NC}"
echo -e "\nCanister UIs:"
echo -e "EMR Registry: ${GREEN}$emr_registry_ui${NC}"
echo -e "Provider Registry: ${GREEN}$provider_registry_ui${NC}"
echo -e "Patient Registry: ${GREEN}$patient_registry_ui${NC}"

echo -e "\n${YELLOW}[IMPORTANT]${NC} Make sure your VPS firewall allows incoming traffic on ports 4943 and 8000\n"
