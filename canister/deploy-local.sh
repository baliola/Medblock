#! /usr/bin/bash

# This script deploys the canister locally.
FE_PORT=4943
lsof -i tcp:${FE_PORT} | awk 'NR!=1 {print $2}' | xargs kill || true

echo "updating did definition"
cargo test --package medblock --lib -- tests --nocapture >/dev/null 2>&1
echo "done"

dfx stop || true

dfx start --background --clean

echo "creating identity"
# medblock provider
dfx identity new medblock_provider || true >/dev/null 2>&1
PROVIDER=$(dfx identity get-principal --identity medblock_provider)

# medblock patient
dfx identity new medblock_patient || true >/dev/null 2>&1
PATIENT=$(dfx identity get-principal --identity medblock_patient)

# medblock admin
dfx identity new medblock_admin || true >/dev/null 2>&1
ADMIN=$(dfx identity get-principal --identity medblock_admin)

echo "done"

# use admin to deploy canister
dfx identity use medblock_admin

dfx deploy

# echo identity for testing
DEFAULT_IDENTITY="2vxsx-fae"

echo "TEST IDENTITY"
echo "default identity: $DEFAULT_IDENTITY"
echo "medblock_provider: $PROVIDER"
echo "medblock_patient: $PATIENT"
echo "medblock_admin (used to deploy canister): $ADMIN"

NIK="3fe93da886732fd563ba71f136f10dffc6a8955f911b36064b9e01b32f8af709"
echo "dummy sha3 nik: $NIK"

EMR=\"{\\\"diagnosis\\\":\\\"cancer\\\",\\\"treatment\\\":\\\"chemotheraphy\\\"}\"
echo "dummy emr records: $EMR"
