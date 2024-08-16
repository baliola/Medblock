# Medblock Technical Documentation

## Overview

Medblock is an Electronic Medical Record (EMR) registry system designed to operate on the Internet Computer. The project is structured into three distinct canisters, each serving a specific function:

1. **Patient Registry**: Manages patient information and records.
2. **EMR Registry**: Centralizes electronic medical records for efficient access and management.
3. **Provider Registry**: Handles information related to healthcare providers.

## Setting Up the Project

Before you begin, ensure that you have the following tools installed on your system:

- **dfx**: The DFINITY SDK for building, deploying, and managing canisters. Refer to the [DFINITY documentation](https://internetcomputer.org/docs/current/developer-docs/getting-started/install/) for installation instructions.
- **candid-extractor**: Install this tool by running `cargo install candid-extractor`.
- **Rust wasm32 target**: Add this target by executing `rustup target add wasm32-unknown-unknown`.
- **ic-wasm**: Install by running `cargo install ic-wasm`.

### Initial Setup

For the initial setup, execute the `setup.sh` script. This script performs several tasks:

1. Verifies the installation of required tools.
2. Creates canister IDs.
3. Generates canister bindings.
4. Compiles all 3 (three) canisters.

> **Note**: The setup process may take some time, depending on your machine's specifications.

## Development Workflow

When developing a feature for a canister, use the `build.sh` script. This script automatically regenerates the candid interface for the canister you're working on and recompiles the EMR registry canister, as the other two canisters depend on it. If you've already run the setup script, this step should be quick. Use the `--all` flag with the script to rebuild all canisters simultaneously.

## Deploy Locally

To deploy the canisters locally, navigate to the `scripts/deployments` directory and execute the `local.sh` script:

```bash
cd scripts/deployments
./local.sh
```

This script performs the following actions:

1. Removes any previous instance of `dfx`.
2. Runs `setup.sh` to ensure all dependencies are installed.
3. Deploys the three canisters using your current `dfx` identity.
4. Links all three canisters together.
5. Adds the anonymous caller as an authorized caller.

## Running Tests

### Unit Tests

To run unit tests, compile and execute them using the `--release` flag to ensure optimal performance:

```bash
cd canister
cargo test --release
```

### Integration Tests

For integration testing, install `pocket-ic`. After installation, follow these steps:

1. Build all canisters using the `build.sh` script with the `--all` flag. This step is necessary after making any changes to the canister code, as running integration tests directly will not recompile the canisters.

   ```bash
   ./build.sh --all
   ```

2. Compile and run the integration tests:

   ```bash
   cd canister/tests/integration-tests
   cargo test --release
   ```

By following this documentation, you can effectively set up, develop, deploy, and test the Medblock EMR registry system on the Internet Computer.
```

This Markdown document is structured to provide clear and concise instructions for setting up and managing the Medblock project.
