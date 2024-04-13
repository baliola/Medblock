# TODO
#! bash
root=$(git rev-parse --show-toplevel)
cd $root/canister
caller=$1
display_name=$2
address=$3
network=$4

if [ -z "$caller" ] || [ -z "$display_name" ] || [ -z "$network" ]; then
    echo "Usage: $0 <caller> <display_name> <address> <network>"
    exit 1
fi

dfx canister call --network=$network provider_registry register_new_provider --type idl "(record {provider_principal=principal \"$caller\"; display_name=\"$display_name\"; address=\"$address\"})" -vv
