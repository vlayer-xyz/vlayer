# Vlayer Web Example

## Building

```sh
forge soldeer install
forge build

cd vlayer
bun install
```

## Running

Run anvil:

```sh
anvil
```

Deploy `WebProofProver` and `WebProofVerifier` contracts on anvil:

```sh
cd vlayer
bun run deploy.ts
```

Start web app on localhost:

```sh
cd vlayer
bun run dev
```

App available at `http://localhost:5174/`

Browser extension should be installed.
