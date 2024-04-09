# Utility scripts.

## Adding canister controller
To use the `add_controller.sh` script, you need to provide three arguments: `canister-name`, `controller-principal`, and `network`.
to see available network go to the `dfx.json`

Here's the general form of the command:

```bash
add_controller.sh <canister-name> <controller-principal> <network>
```

## Adding Authorized Metrics Collector
This script is used to add an authorized metrics collector to a specified canister. It uses the `dfx canister call` command to call the `add_authorized_metrics_collector` method on the canister.

To use the `add_metrics_collector.sh` script, you need to provide three arguments: `canister-name`, `caller`, and `network`.

Here's the general form of the command:

```bash
./add_metrics_collector.sh <canister-name> <caller> <network>
```

## Add Provider
This script is used to register a new provider in the provider registry. It uses the `dfx canister call` command to call the `register_new_provider` method on the provider registry canister.

To use the `add_provider.sh` script, you need to provide three arguments: `caller`, `display_name`, and `network`.

Here's the general form of the command:

```bash
./add_provider.sh <caller> <display_name> <network>
```