# Contributing to vlayer

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

## Developer experience

Incremental compilation in this repo is slow by default because it re-compiles `guest_wrapper` to `RISC-V` target to generate up to date `GUEST_ELF`.
Most developer workflows don't need this and therefore would benefit from temporarily disabling this step.

Examples on when you should disable it:
* Getting something to type-check/compile
* Working on unit-tests of functionality that does not call any guest code through host

Examples when you should not disable it:
* CI
* Integration/E2E tests that call guest through host

To disable guest build - set the flag: `RISC0_SKIP_BUILD = "1"` in `.cargo/config.toml`. This file is respected by both rust-analyzer in the IDE as well as CLI commands
