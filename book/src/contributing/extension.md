# Contributing to vlayer browser extension

## Prerequisites

To start working with vlayer browser extension, you need to install following software:

- [Bun](https://bun.sh/)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)

## Running

Run anvil:

```sh
anvil
```

Run vlayer server:

```sh
cargo run vlayer serve
```

Deploy WebProofProver and WebProofVerifier contracts on anvil:

```sh
cd vlayer/examples/web_proof/vlayer
bun run deploy.ts
```

`deploy.ts` script deploys prover and verifier contracts. These contracts' addresses are saved in `.env.development` file and later used by the web app.

Start web app on localhost:

```sh
cd vlayer/examples/web_proof/vlayer
bun run dev
```

Start browser extension:

```sh
cd vlayer/packages/browser-extension
bun run dev
```

This will open web browser with vlayer app and browser extension installed.

Now all saved changes will be applied in browser.

## Vlayer web app

Web app's files are in `vlayer/examples/web_proof/vlayer` folder.

## Vlayer browser extension

Extension's files are in `vlayer/packages/browser-extension` folder.
