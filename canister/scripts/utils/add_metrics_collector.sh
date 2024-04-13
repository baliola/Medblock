#! bash
root=$(git rev-parse --show-toplevel)
cd $root/canister
canister=$1
caller=$2
network=$3

if [ -z "$canister" ] || [ -z "$caller" ] || [ -z "$network" ]; then
    echo "Usage: $0 <canister> <caller> <network>"
    exit 1
fi

dfx canister call --network=$network $canister add_authorized_metrics_collector --type idl "(record {caller=principal \"$caller\" })" -vv
