# TODO
#! bash
root=$(git rev-parse --show-toplevel)
caller=$1
display_name=$2
network=$3

if [ -z "$caller" ] || [ -z "$display_name" ] || [ -z "$network" ]; then
    echo "Usage: $0 <caller> <display_name> <network>"
    exit 1
fi

dfx canister call --network=$network provider_registry register_new_provider --type idl "(record {provider_principal=principal \"$caller\"; display_name=\"$display_name\"})" -vv
