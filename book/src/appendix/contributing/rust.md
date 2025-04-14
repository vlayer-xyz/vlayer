# Contributing to vlayer Rust codebase

## Prerequisites

To start working with [this repository](https://github.com/vlayer-xyz/vlayer), you will need to install following software:

- [Rust](https://www.rust-lang.org/tools/install) compiler
- [Risc-0 toolchain](https://dev.risczero.com/api/zkvm/quickstart) with VM version v2.0.1 and Rust toolchain version v1.81.0
  ```
  curl -L https://risczero.com/install | bash
  export PATH=$PATH:~/.risc/bin
  rzup install r0vm 2.0.1
  rzup install rust 1.81.0
  ```
- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [Bun](https://bun.sh) and [Node.js](https://nodejs.org)
- [LLVM Clang](https://clang.llvm.org/) compiler version which supports [RISC-V build target](https://llvm.org/docs/RISCVUsage.html) available on the `PATH`
- `timeout` terminal command

### Homebrew

If you're on macOS, you can use [Homebrew](https://brew.sh/) to install some of the dependencies:

#### LLVM
```
brew install llvm
```
After installation, make sure to follow the instructions to update your `PATH`.

#### coretils
```
brew install coreutils
```

## Building vlayer
 
In this guide, we will focus on running `examples/simple` example.

### Build solidity smart contracts

First, make sure the dependencies are up-to-date:

```sh
git submodule update --init --recursive
```

Next, navigate to `contracts/vlayer` directory, and run:

```sh
cd contracts/vlayer
forge soldeer install
```

###  Build vlayer proving server

To build the project, first, navigate to the `rust` directory and run:

```sh
cd rust
cargo build
```

### Build JS/TS SDK

Navigate to `packages` directory and run:

```sh
cd packages
bun install
```

Next, navigate to `packages/sdk` directory and run:

```sh
cd packages/sdk
bun run build
```

## Running example


### Run Anvil

Open a new terminal and run:

```sh
anvil
```

### Run vlayer proving server

Open a new terminal, navigate to `rust` directory and run:

```sh
RUST_LOG=info cargo run --bin call_server -- --rpc-url '31337:http://localhost:8545' --proof fake
```

### Build example contracts

Finally, to test proving navigate to `examples/simple` directory, and run following commands to build example's contracts: 

```sh
forge soldeer install
forge clean 
forge build
```

Next, navigate to `vlayer` directory, and run the following command:

```sh
bun install 
bun run prove:dev
``` 

For guides about the project structure, check out [architecture appendix](/appendix/architecture/overview.html).

## Guest Profiling

To profile execution of Guest code in zkVM, we leverage the profiling functionality [provided by RISC Zero](https://dev.risczero.com/api/zkvm/profiling). In order to run profiling, follow the steps in the [Usage](https://dev.risczero.com/api/zkvm/profiling#usage) section of the RISC Zero documentation, but in [Step 2](https://dev.risczero.com/api/zkvm/profiling#step-2-running) replace the command you run with:

```sh
RISC0_PPROF_OUT=./profile.pb cargo run --bin call_server --proof fake
```

which will start the vlayer server. Then just call the JSON RPC API and the server will write the profiling output to `profile.pb`, which can be later [visualised as explained in the RISC Zero Profiling Guide](https://dev.risczero.com/api/zkvm/profiling#step-3-visualization). Please note that the profile only contains data about the Guest execution, i.e. the execution inside the zkVM.

## Working with guest ELF IDs

Dockerized guest builds ensure that guest ELF IDs remain deterministic. This process is managed by the build script in the `rust/guest_wrapper` crate, which relies on the `build-utils` crate. Current and historical chain guest IDs are stored in the repository to maintain consistency when calling host and guest functions (see Repository Artifacts below).

### Generating `ImageID.sol`

The guest wrapper's build script generates the file `target/assets/ImageID.sol`, which is symlinked to `contracts/vlayer/src/ImageID.sol`.
If contract compilation fails due to a missing `ImageID.sol`, run:

    cargo build

Additionally, remember to recompile contracts after rebuilding the guest.

### Running end-to-end tests

To run end-to-end tests with real chain workers, a dockerized build must be completed in advance. This is done by compiling with:

    RISC0_USE_DOCKER=1 cargo build

This process typically takes 4-5 minutes on a MacBook Pro (using Apple Virtualization with Rosetta for amd64 emulation).

### Handling `Chain guest ELF ID mismatch` errors

If a dockerized build fails with a `Chain guest ELF ID mismatch` error, it means the chain guest has changed, and the ELF ID must be updated. To resolve this, re-run the build with:

    RISC0_USE_DOCKER=1 UPDATE_GUEST_ELF_ID=1 cargo build

This will:
  1. Move the previous chain guest ELF ID to the historical IDs file,
  2. Put the new chain guest ELF ID (generated during the compilation) into the file with current ELF ID,
  3. Generate a TODO changelog entry, which should be consequently filled in with change description by the person introducing the change.

### Repository artifacts

  1. `/rust/guest_wrapper/artifacts/chain/elf_id`  – single-line text file with hex-encoded ELF ID of the current chain guest. No trailing newline.
  2. `/rust/guest_wrapper/artifacts/chain/elf_id_history` – multi-line text file with all historical chain guest IDs, hex-encoded, one ID per line, sorted from oldest to newest, initially empty.
  3. `/rust/guest_wrapper/artifacts/chain/CHANGELOG.md` – markdown file where every chain guest ID (including current one) is annotated with creation date and a list of changes.

## Troubleshooting

### Error on macOS while `cargo build`: `assert.h` file doesn't exist

In some cases while running `cargo build`, an error occurs with compiling `mdbx-sys`.  
In that case install `xcode-select`:
``` sh
xcode-select --install
```
If you get the message `Command line tools are already installed`, but the problem persists, reinstall it:
``` sh
sudo rm -rf /Library/Developer/CommandLineTools
xcode-select --install
```
Then, install updates by "Software Update" in System Settings and finally restart your computer.
