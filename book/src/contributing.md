# Contributing


## Prerequisites

You will need to install following software to start working with repository:

- [Rust](https://www.rust-lang.org/tools/install) compiler
- Rust risc-0 [toolchain](https://dev.risczero.com/api/zkvm/quickstart)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)

## Building

To build project navigate to `rust` directory and type:

```sh
cargo build
```

## Running

To deploy contract first install `jq`:

```sh
brew install jq
```

Run anvil in the background: 
```sh
anvil
```

Deploy contract by going to its directory (e.g. `examples/simple`) and run `../../bash/vlayer-deploy.sh`.
If `VLAYER_CONTRACT_ADDRESS` is displayed, contract was deployed successfully.

Finally, run:

```sh
RUST_LOG=info RISC0_DEV_MODE=1 cargo run
```

## Book

To build book you will need mdbook installed:
```sh
cargo install mdbook
```

To compile diagrams in the book, you need to install [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid) preprocessor:
```sh
cargo install mdbook-mermaid
```

To build book navigate to `book/` and type:
```
mdbook serve
```

Book is available at `http://localhost:3000/`.