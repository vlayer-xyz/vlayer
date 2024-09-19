# Contributing to vlayer browser extension

## Prerequisites

To start working with vlayer browser extension, you need to install following software:

- [Bun](https://bun.sh/)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)

## Building

```sh
cd rust
cargo build --release
```

```sh
cd contracts
forge soldeer install
forge clean
forge build
```

Web app's files are in `examples/web_proof/vlayer` folder.

```sh
cd examples/web_proof
forge soldeer install
forge clean
forge build
```

```sh
cd examples/web_proof/vlayer
bun install
```

Extension's files are in `packages/browser-extension` folder.

```sh
cd packages
bun install
```

```sh
cd packages/vlayer/sdk/src
bun install
```

## Running

Run anvil:

```sh
anvil
```

Run vlayer server:

```sh
cd rust
cargo run --bin vlayer serve --proof fake
```

Deploy `WebProofProver` and `WebProofVerifier` contracts on anvil:

```sh
cd examples/web_proof/vlayer
bun run deploy.ts
```

`deploy.ts` script deploys prover and verifier contracts. These contracts' addresses are saved in `.env.development` file and later used by the web app.

Start web app on localhost:

```sh
cd examples/web_proof/vlayer
bun run dev
```

Before starting browser extension, copy `.env.template` to `.env.development` file in `browser-extension` directory.
Start browser extension:

```sh
cd packages/browser-extension
bun run dev
```

This will open web browser with vlayer app and browser extension installed.
Now all saved changes will be applied in browser.

There is a script `bash/run-web-example.sh` that runs all of the above mentioned steps.

### Extension watch mode

Extension can be also built using:

```sh
bun run build:watch
```

in `packages/browser-extension` directory. It enables hot-reload of the extension.
