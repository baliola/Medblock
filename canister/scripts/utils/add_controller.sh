#! bash
root=$(git rev-parse --show-toplevel)
cd $root/canister
canister=$1
controller=$2
network=$3

if [ -z "$canister" ] || [ -z "$controller" ] || [ -z "$network" ]; then
    echo "Usage: $0 <canister> <controller> <network>"
    exit 1
fi

dfx canister update-settings $canister --add-controller $controller --network=$network
