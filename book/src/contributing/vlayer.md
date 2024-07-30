# Contributing to vlayer

## Prerequisites

To start working with this repository, you will need to install following software:

- [Rust](https://www.rust-lang.org/tools/install) compiler
- Rust risc-0 [toolchain](https://dev.risczero.com/api/zkvm/quickstart)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [Bun](https://bun.sh)

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

