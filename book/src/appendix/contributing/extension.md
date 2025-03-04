# Contributing to the vlayer browser extension

## Prerequisites

To start working with the vlayer browser extension, you need to install the following software:

- [Bun](https://bun.sh/)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [Chrome Browser](https://www.google.com/chrome/)

## Building

First build the vlayer server with:

```sh
cd rust
cargo build
```

 Then build vlayer contracts with:

```sh
cd contracts
forge soldeer install
forge clean
forge build
```

Web app's files are in `examples/simple_web_proof/vlayer` folder.

```sh
cd examples/simple_web_proof
forge soldeer install
forge clean
forge build
```

```sh
cd examples/simple_web_proof/vlayer
bun install
```

Extension's files are in `packages/browser-extension` folder.

```sh
cd packages
bun install
```

## Local development

Run anvil:

```sh
anvil
```

Run the vlayer server:

```sh
cd rust
cargo run --bin call_server --proof fake
```

Deploy `WebProofProver` and `WebProofVerifier` contracts on anvil:

```sh
cd examples/simple_web_proof/vlayer
bun run deploy.ts
```

`deploy.ts` script deploys the Prover and Verifier contracts. Their addresses are saved in the `.env.development` file and later used by the web app.

Start web app on localhost:

```sh
cd examples/simple_web_proof/vlayer
bun run web:dev
```

Then, start the browser extension:

```sh
cd packages/browser-extension
bun run dev
```

This will open a web browser with the vlayer app and browser extension installed.
Now all the saved changes will be applied in your browser automatically.

There is [a script](https://github.com/vlayer-xyz/vlayer/blob/main/bash/run-web-example.sh), that runs all of the steps above.

### Extension watch mode

Extension can be also built using:

```sh
bun run build:watch
```

in `packages/browser-extension` directory. It enables hot-reload of the extension.


## Testing

Extension end-to-end tests are stored in `packages/browser-extension/tests` folder.  

Testing uses Playwright web testing library. Install it with:
```sh
bunx playwright install --with-deps chromium
```

To run tests, firstly, install Typescript dependencies in `packages` folder:
```sh
cd packages
bun install
```

Then, build the extension:
```sh
cd packages/browser-extension
bun run build
```

Finally, run tests:
```sh
cd packages/browser-extension
bun run test:headless
```
