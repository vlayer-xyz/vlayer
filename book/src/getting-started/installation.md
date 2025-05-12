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
