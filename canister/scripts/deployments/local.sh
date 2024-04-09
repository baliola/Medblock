#! bash
root=$(git rev-parse --show-toplevel)

bash $root/canister/setup.sh
# This script deploys the canister locally.
FE_PORT=4943
lsof -i tcp:${FE_PORT} | awk 'NR!=1 {print $2}' | xargs kill || true

dfx start --background
dfx canister install emr_registry --wasm $root/canister/target/wasm32-unknown-unknown/release/emr_registry.wasm --mode=install -y
# idk why but the candid ui wont load unless we rebuild patient registry
bash $root/canister/build.sh patient_registry
dfx canister install patient_registry --wasm $root/canister/target/wasm32-unknown-unknown/release/patient_registry.wasm --mode=install -y
dfx canister install provider_registry --wasm $root/canister/target/wasm32-unknown-unknown/release/provider_registry.wasm --mode=install -y

DEFAULT_IDENTITY="2vxsx-fae"
emr_registry_id=$(dfx canister id emr_registry)
patient_registry_id=$(dfx canister id patient_registry)
provider_registry_id=$(dfx canister id provider_registry)
# TODO : bind them to each other

echo "adding anonymous principal as controllers (for easier testing lol)"
bash $root/canister/scripts/utils/add_controller.sh emr_registry "$DEFAULT_IDENTITY" local
bash $root/canister/scripts/utils/add_metrics_collector.sh emr_registry "$DEFAULT_IDENTITY" local
echo "adding authorized caller to emr  canister"
dfx canister call --network=local emr_registry add_authorized_caller --type idl "(record {caller=principal \"$provider_registry_id\" })" --candid $root/canister/src/emr_registry/candid.did
dfx canister call --network=local emr_registry add_authorized_caller --type idl "(record {caller=principal \"$patient_registry_id\" })" --candid $root/canister/src/emr_registry/candid.did

bash $root/canister/scripts/utils/add_controller.sh patient_registry "$DEFAULT_IDENTITY" local
bash $root/canister/scripts/utils/add_metrics_collector.sh patient_registry "$DEFAULT_IDENTITY" local
echo "adding authorized caller to patient canister"
dfx canister call --network=local patient_registry update_emr_registry_principal --type idl "(record {\"principal\"=principal \"$emr_registry_id\" })" --candid $root/canister/src/patient_registry/candid.did
dfx canister call --network=local patient_registry update_provider_registry_principal --type idl "(record {\"principal\"=principal \"$provider_registry_id\" })" --candid $root/canister/src/patient_registry/candid.did

bash $root/canister/scripts/utils/add_controller.sh provider_registry "$DEFAULT_IDENTITY" local
bash $root/canister/scripts/utils/add_metrics_collector.sh provider_registry "$DEFAULT_IDENTITY" local
echo "adding authorized caller to provider canister"
dfx canister call --network=local provider_registry update_emr_registry_principal --type idl "(record {\"principal\"=principal \"$emr_registry_id\" })" --candid $root/canister/src/provider_registry/candid.did
dfx canister call --network=local provider_registry update_patient_registry_principal --type idl "(record {\"principal\"=principal \"$patient_registry_id\" })" --candid $root/canister/src/provider_registry/candid.did

# echo identity for testing
candid_ui=$(dfx canister id __Candid_UI)
basepath="http://127.0.0.1:4943/?canisterId=$candid_ui"
emr_registry_ui="$basepath&id=$emr_registry_id"
provider_registry_ui="$basepath&id=$provider_registry_id"
patient_registry_ui="$basepath&id=$patient_registry_id"
NIK="3fe93da886732fd563ba71f136f10dffc6a8955f911b36064b9e01b32f8af709"

echo "default identity: $DEFAULT_IDENTITY"
echo
echo "dummy nik: $NIK"
echo "emr registry ui: $emr_registry_ui"
echo
echo "provider registry ui: $provider_registry_ui"
echo
echo "patient registry ui: $patient_registry_ui"
echo
