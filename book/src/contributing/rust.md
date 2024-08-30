# Contributing to vlayer Rust codebase

## Prerequisites

To start working with this repository, you will need to install following software:

- [Rust](https://www.rust-lang.org/tools/install) compiler
- Rust risc-0 [toolchain](https://dev.risczero.com/api/zkvm/quickstart)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [Bun](https://bun.sh)
- [LLVM Clang](https://clang.llvm.org/) compiler version which supports [RISC-V build target](https://llvm.org/docs/RISCVUsage.html) available on the `PATH`

## Building

Before you build solidity smart contracts, make sure that dependencies are up to date:
```sh
git submodule update --init --recursive
```

To build vlayer project, first, navigate to the `rust` directory and type:

```sh
cargo build
```

## Running

Run anvil in the background:
```sh
anvil
```

Then, to run proving server, execute the following command:
```sh
RUST_LOG=info RISC0_DEV_MODE=1 cargo run -- serve
```

Finally, to test proving navigate to any of the examples within `/examples` directory, find `vlayer` directory and run the following command:
```sh
forge clean 
forge build
bun install 
bun run prove.ts
``` 

For guides about the project structure, check out [architecture appendix](/appendix/architecture.md).

## Guest Profiling

To profile execution of Guest code in zkVM, we leverage the profiling functionality [provided by risc0](https://dev.risczero.com/api/zkvm/profiling). In order to run profiling, follow the steps in the [Usage](https://dev.risczero.com/api/zkvm/profiling#usage) section of the risc0 documentation, but in [Step 2](https://dev.risczero.com/api/zkvm/profiling#step-2-running) replace the command you run with:

```sh
RISC0_PPROF_OUT=./profile.pb cargo run --bin vlayer serve --proof fake
```

which will start the vlayer server. Then just call the JSON RPC API and the server will write the profiling output to `profile.pb`, which can be later [visualised as explained in the risc0 Profiling Guide](https://dev.risczero.com/api/zkvm/profiling#step-3-visualization). Please note that the profile only contains data about the Guest execution, i.e. the execution inside the zkVM.
