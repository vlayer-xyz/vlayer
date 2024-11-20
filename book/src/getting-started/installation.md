# Installation
The easiest way to install vlayer is by using `vlayerup`, the vlayer toolchain installer.

## Supported Platforms
Operating System: Only Ubuntu 24.04 LTS / macOS 14  or newer versions are supported. Other Linux distributions may work but are not officially supported.
Hardware: Macs with Intel CPUs are not supported. Use an M1/M2/M3 Mac or a supported Linux machine instead.

## Prerequisites
Before working with vlayer, ensure the following tools are installed:
- [Git](https://git-scm.com/downloads)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [RISC Zero](https://dev.risczero.com/api/zkvm/install) in version 1.0.5
  - `curl -L https://risczero.com/install | bash`
  - `rzup install cargo-risczero v1.0.5`
  - `cargo risczero install` 

Additionally, you'll need [Bun](https://bun.sh/) to run examples. For more details, refer to the [Running Examples Locally](/getting-started/first-steps.html#running-examples-locally) section.

## Get vlayerup

To install `vlayerup`, run the following command in your terminal, then follow the onscreen instructions.
```sh
curl -SL https://install.vlayer.xyz | bash
```

This will install `vlayerup` and make it available in your CLI.

## Using vlayerup
Running `vlayerup` will install the latest (nightly) precompiled binary of vlayer:
```sh
vlayerup
```

You can check that the binary has been successfully installed and inspect its version by running:

```sh
vlayer --version
```
