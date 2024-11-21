# First steps with vlayer

## Creating a new project

Run this command to initialize a new vlayer project:
```bash
vlayer init project-name
```

It creates a folder with sample contracts.

### Adding to an existing project
Use the `--existing` flag to initialize vlayer within your existing [Foundry](https://getfoundry.sh/) project:
```bash
cd ./your-project && vlayer init --existing
```

### Example project

To initialize a vlayer project with example prover and verifier contracts, use the `--template` flag as shown below:

```bash
vlayer init simple --template simple
```

The following templates are available for quick project setup:

- `simple`: Prove an ERC20 token balance at a specific block number.
- `simple-email`: Mint an NFT to the owner of an email address from a specific domain.
- `simple-teleport`: Prove a cross-chain ERC20 token balance.
- `simple-time-travel`: Prove the average ERC20 token balance across multiple block numbers.
- `web-proof`: Mint an NFT to the owner of a specific X/Twitter handle using Web Proofs.
- `email-proof`: Link an email address to a wallet address by verifying the email subject.

## Directory structure
The vlayer directory structure resembles a typical Foundry project but with two additional folders: `src/vlayer` and `vlayer`.
* `src/vlayer`: Contains the Prover and Verifier smart contracts.
* `vlayer`: Has contract deployment scripts, client SDK calls to the prover, and verifier transactions.
 

## Running examples locally

### All examples
> ❗️ Make sure that you have [Bun](https://bun.sh/) installed in your system to build and run the examples.

To run vlayer examples locally, first build the contracts by navigating to your project folder and running:
```bash
cd your-project
forge build
```
This compiles the smart contracts and prepares them for deployment and testing.

> Please note that `vlayer init` installs Solidity dependencies and generates `remappings.txt`. Running `forge soldeer install` is not needed to build the example and may overwrite remappings, which can cause build errors.

Then, install Typescript dependencies in vlayer folder by running:
```bash
cd vlayer
bun install
```

### Run on a local devnet
To set up a local development environment, first launch a local Ethereum node:

```bash
$ anvil 
```

Then, in a separate terminal, start the [Prover server](/advanced/prover.html#prover-server):

```bash
vlayer serve
```

If you're using Prover features like teleport or time travel, configure the appropriate JSON-RPC URLs for each chain. You can use Alchemy or other providers to supply these URLs:

```bash
vlayer serve \
  --rpc-url '31337:http://localhost:8545' \
  --rpc-url '11155111:https://eth-sepolia.g.alchemy.com/v2/{ALCHEMY_KEY}' \
  --rpc-url '1:https://eth-mainnet.g.alchemy.com/v2/{ALCHEMY_KEY}' \
  --rpc-url '8453:https://base-mainnet.g.alchemy.com/v2/{ALCHEMY_KEY}' \
  --rpc-url '10:https://opt-mainnet.g.alchemy.com/v2/{ALCHEMY_KEY}'
```

To run the example within the `vlayer` directory, use:

```sh
bun run prove:dev
```

### Run on a testnet
To use a testnet, first provide a private key in the `vlayer/.env.testnet.local` file:

```sh
EXAMPLES_TEST_PRIVATE_KEY=0x....
```

This private key is used for deploying example contracts and sending transactions to the verifier.

By default, `optimismSepolia` is configured in the `vlayer/.env.testnet` file. However, you can override this setting to use other testnets. Below is a list of available testnets and their respective JSON-RPC URLs:

| CHAIN_NAME        | JSON_RPC_URL                                  |
|-------------------|-----------------------------------------------|
| sepolia           | https://rpc.sepolia.org                       |
| optimismSepolia   | https://sepolia.optimism.io                   |
| baseSepolia       | https://sepolia.base.org                      |
| polygonAmoy       | https://rpc-amoy.polygon.technology           |
| arbitrumSepolia   | https://sepolia-rollup.arbitrum.io/rpc        |
| zksyncSepoliaTestnet | https://sepolia.era.zksync.dev            |
| flowTestnet       | https://testnet.evm.nodes.onflow.org          |
| scrollSepolia     | https://sepolia-rpc.scroll.io                 |
| lineaSepolia      | https://rpc.sepolia.linea.build               |
| bitkubTestnet     | https://rpc-testnet.bitkubchain.io            |
| zircuitTestnet    | https://zircuit1.p2pify.com                   |

To change the desired network, set the `CHAIN_NAME` and `JSON_RPC_URL` environment variables in `vlayer/.env.testnet.local`.

Once configured, run the example from within the `vlayer` directory using:

```sh
bun run prove:testnet
```

### Web Proof example

First, install the vlayer browser extension from the [Chrome Web Store](https://chromewebstore.google.com/detail/vlayer/jbchhcgphfokabmfacnkafoeeeppjmpl) (works with Chrome and Brave browsers). For more details about the extension, see the [Web Proofs](../javascript/web-proofs.md) section.

Then deploy the `WebProofProver` and `WebProofVerifier` contracts:

```sh
cd vlayer
bun run deploy:dev # deploy to local anvil
bun run deploy:testnet # deploy to testnet
```

Start web app on localhost:

```sh
cd vlayer
bun run dev
```

The app will be available at `http://localhost:5174` and will display buttons that will let you interact with the extension and vlayer server (open browser developer console to see the app activity).
