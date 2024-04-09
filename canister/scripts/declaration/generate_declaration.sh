#! bash

root=$(git rev-parse --show-toplevel)
out=$1

if [ -z "$out" ]; then
    echo "Usage: $0 <output_dir>"
    exit 0
fi

echo generating declarations
cd $root/canister
dfx generate patient_registry
dfx generate emr_registry
dfx generate provider_registry

echo moving declarations to $out
rm -rf $out/declarations
mv $root/canister/src/declarations $out/
