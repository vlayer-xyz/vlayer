# Contributing to vlayer Rust codebase

## Prerequisites

To start working with [this repository](https://github.com/vlayer-xyz/vlayer), you will need to install following software:

- [Rust](https://www.rust-lang.org/tools/install) compiler
- Rust risc-0 [toolchain](https://dev.risczero.com/api/zkvm/quickstart) version v1.2.0
  ```
  rzup install cargo-risczero v1.2.0
  cargo risczero install
  ```
- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [Bun](https://bun.sh) and [Node.js](https://nodejs.org)
- [LLVM Clang](https://clang.llvm.org/) compiler version which supports [RISC-V build target](https://llvm.org/docs/RISCVUsage.html) available on the `PATH`
- `timeout` terminal command (`brew install coreutils` on macOS)

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
RUST_LOG=info RISC0_DEV_MODE=1 cargo run --bin call_server -- --rpc-url '31337:http://localhost:8545'
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

We are using Dockerized guest builds to ensure that ELF IDs of guests are deterministic. This is handled by the build script of `rust/guest_wapper` crate, which uses `build-utils` crate. Current and historical chain guest IDs are stored in repository so that they can be safely included in call host & guest (see “Repository artifacts” below).

⚠️ The build script of guest wrapper generates `target/assets/ImageID.sol` file which is symlinked into `contracts/vlayer/src/ImageID.sol` . Run `cargo build` to generate it, if compiling contracts fails due to `ImageID.sol` missing. Also, remember to recompile contracts after rebuilding the guest.

⚠️ For running e2e tests with real chain workers, a dockerized build must be done beforehand. This is achieved by building the code with `RISC0_USE_DOCKER=1`. This usually takes ~4-5 minutes (on MacBook Pro with Apple Virtualization, using Rosetta for amd64 emulation). 

⚠️ If a dockerized build fails with `Chain guest ELF ID mismatch` error, this indicates that some changes have been made to the chain guest and the ELF ID needs to be updated. This is done by re-running the build with `RISC0_USE_DOCKER=1 UPDATE_GUEST_ELF_ID=1` flags. It will do the following:

  1. Move the previous chain guest ELF ID to the file with historical IDs,
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
