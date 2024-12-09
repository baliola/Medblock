#!/bin/bash

# get project root
root=$(git rev-parse --show-toplevel)

# handle output paths
declare -a output_paths=(
    "$root/final_demo/pwa/src"
    "$root/final_demo/web/src"
    "$root/internal-dashboard/src/canister"
)

echo "generating declarations..."
cd $root/canister

# generate declarations for all canisters
dfx generate patient_registry
dfx generate emr_registry
dfx generate provider_registry

# copy declarations to each output path
for out in "${output_paths[@]}"; do
    echo "copying declarations to $out"
    rm -rf "$out/declarations"
    cp -r "$root/canister/src/declarations" "$out/"
done

echo "declaration generation complete!"
