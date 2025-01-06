#! bash
root=$(git rev-parse --show-toplevel)
cd $root/canister
controller=$1
network=$2

if [ -z "$controller" ] || [ -z "$network" ]; then
    echo "Usage: $0 <controller> <network>"
    exit 1
fi

echo "⚠️  WARNING ⚠️"
echo "You are about to add a controller to multiple canisters."
echo "Each canister can only have a MAXIMUM of 10 controllers total."
echo "Once added, controllers cannot be easily removed."
echo "Are you sure you want to proceed? (y/N): "
read -r response

if [[ ! "$response" =~ ^[Yy]$ ]]; then
    echo "Operation cancelled."
    exit 1
fi

# read canister_ids.json and extract canister names
canister_names=$(jq -r 'keys[]' canister_ids.json)

# loop through each canister and add controller
for canister in $canister_names; do
    echo "Processing $canister..."
    
    # get current controllers
    current_controllers=$(dfx canister status $canister --network=$network | grep "Controllers:" | cut -d':' -f2)
    controller_count=$(echo $current_controllers | wc -w)
    
    if [ $controller_count -ge 10 ]; then
        echo "⚠️  Warning: $canister already has $controller_count controllers (maximum is 10). Skipping..."
        continue
    fi
    
    # check if controller is already added
    if echo "$current_controllers" | grep -q "$controller"; then
        echo "👍 Controller already exists for $canister. Skipping..."
        continue
    fi
    
    echo "Adding controller to $canister..."
    dfx canister update-settings $canister --add-controller $controller --network=$network
done
