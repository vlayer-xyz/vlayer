# Contributing to vlayer browser extension

## Prerequisites

To start working with vlayer browser extension, you need to install following software:

- [Bun](https://bun.sh/)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)

## Building

First build vlayer server with:

```sh
cd rust
cargo build --release
```

Then build vlayer contracts with:

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

## Local development

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

`deploy.ts` script deploys the Prover and Verifier contracts. Their addresses are saved in the `.env.development` file and later used by the web app.

Start web app on localhost:

```sh
cd examples/web_proof/vlayer
bun run dev
```

Before starting the browser extension, copy the `.env.template` file to `.env.development` in the `browser-extension` directory.
Then, start the browser extension:

```sh
cd packages/browser-extension
bun run dev
```

This will open a web browser with the vlayer app and browser extension installed.
Now all saved changes will be applied in browser.

There is a script, `bash/run-web-example.sh`, that runs all of the above steps.

### Extension watch mode

Extension can be also built using:

```sh
bun run build:watch
```

in `packages/browser-extension` directory. It enables hot-reload of the extension.
