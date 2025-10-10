# Installation
The easiest way to install vlayer is by using `vlayerup`, the vlayer toolchain installer.

## Supported Platforms
**Linux**: Only Ubuntu 24.04 LTS or newer versions with x86_64 CPU architecture are supported. Other Linux distributions may work but are not officially supported.

**Mac**: Macs with Intel CPUs are not supported. Use a Mac with Apple Silicon.

## Prerequisites
Before working with vlayer, ensure the following tools are installed:
- [Git](https://git-scm.com/downloads)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)

Additionally, you'll need [Bun](https://bun.sh/) to run examples. For more details, refer to the [Running Examples Locally](/getting-started/first-steps.html#running-examples-locally) section.

## Get vlayerup

To install `vlayerup`, run the following command in your terminal, then follow the onscreen instructions.
```sh
curl -SL https://install.vlayer.xyz | bash
```

This will install `vlayerup` and make it available in your CLI.
Occasionally, vlayerup might require an upgrade. To update it, simply run the same command again.

## Using vlayerup
Running `vlayerup` will install the latest (nightly) precompiled binary of vlayer:
```sh
vlayerup
```

You can check that the binary has been successfully installed and inspect its version by running:

```sh
vlayer --version
```

### Installing nightly version

You can use use the following command to install a nightly version of vlayer:

```sh
vlayerup --channel nightly
```

## Using vlayer test

In order to execute `vlayer test` with vlayer [`examples`](https://github.com/vlayer-xyz/vlayer/tree/main/examples), vlayer requires `risc0-ethereum` version 3.0 or higher. Using an older version (< 3.0) will result in proof verification failures due to incompatible control roots and verifier parameters.

To ensure you have the most up-to-date version of the RISC Zero toolchain (cargo-risczero):

```sh
cargo install cargo-risczero --force
```

This installs or updates `cargo-risczero` to the latest version.

You can check that it has been successfully installed and inspect its version by running:

```sh
cargo risczero --version
```