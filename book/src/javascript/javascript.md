# Vanilla JS/TS
## JavaScript
<div class="feature-card feature-in-dev">
  <div class="title">
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M8.57499 3.21665L1.51665 15C1.37113 15.252 1.29413 15.5377 1.29331 15.8288C1.2925 16.1198 1.3679 16.4059 1.51201 16.6588C1.65612 16.9116 1.86392 17.1223 2.11474 17.2699C2.36556 17.4174 2.65065 17.4968 2.94165 17.5H17.0583C17.3493 17.4968 17.6344 17.4174 17.8852 17.2699C18.136 17.1223 18.3439 16.9116 18.488 16.6588C18.6321 16.4059 18.7075 16.1198 18.7067 15.8288C18.7058 15.5377 18.6288 15.252 18.4833 15L11.425 3.21665C11.2764 2.97174 11.0673 2.76925 10.8176 2.62872C10.568 2.48819 10.2864 2.41437 9.99999 2.41437C9.71354 2.41437 9.43193 2.48819 9.18232 2.62872C8.93272 2.76925 8.72355 2.97174 8.57499 3.21665V3.21665Z" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 7.5V10.8333" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 14.1667H10.0083" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
    Actively in Development
  </div>
  <p>Our team is currently working on this feature. If you experience any bugs, please let us know <a href="https://discord.gg/JS6whdessP" target="_blank">on our Discord</a>. We appreciate your patience. </p>
</div>

A common way to interact with blockchain is to make calls and send transactions from JavaScript, most often from a web browser. vlayer provides developer friendly JavaScript/TypeScript API - vlayer SDK. It combines well with the standard way of interacting with smart contracts. 

## Installation
To install vlayer SDK, run the following command in your JavaScript application

{{#tabs }}
{{#tab name="yarn" }}
```sh
yarn add @vlayer/sdk
```
{{#endtab }}
{{#tab name="npm" }}
```sh
npm i @vlayer/sdk
```
{{#endtab }}
{{#tab name="pnpm" }}
```sh
pnpm i @vlayer/sdk
```
{{#endtab }}
{{#tab name="bun" }}
```sh
bun i @vlayer/sdk
```
{{#endtab }}

{{#endtabs }}


## vlayer client

The **vlayer client** provides an interface for interacting with the vlayer JSON-RPC API. It enables you to trigger and monitor proof statuses while offering convenient access to features such as [Web Proofs](https://github.com/vlayer-xyz/vlayer/pull/web-proofs.md) and [Email Proofs](https://github.com/vlayer-xyz/vlayer/pull/email-proofs.md).

### Initializing

You can initialize a client as shown below:

```ts
import { createVlayerClient } from "@vlayer/sdk";

const vlayer = createVlayerClient({
  url: "http://127.0.0.1:3000", // local or remote prover url
});
```

### Deploying contracts
Before proving, you must deploy the Prover and Verifier contracts. You can use the provided function helper or custom deployment scripts.

Example using the ``deployVlayerContracts` helper:
```ts
import proverSpec from "../out/WebProofProver.sol/WebProofProver";
import verifierSpec from "../out/WebProofVerifier.sol/WebProofVerifier";

import { deployVlayerContracts } from "@vlayer/sdk/config";

const { proverAddr, verifierAddr } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
});
```

### Client configuration
vlayer examples use `.env` files for configuration and you may set following parameters:
- `CHAIN_NAME` name of the chain (viem compatible) on which the prover and verifier contracts are deployed
- `PROVER_URL` url of the prover to connect to (more info [here](/advanced/dev-and-production.html))
- `JSON_RPC_URL` url of the JSON-RPC provider used for verifying proofs (optional, by default json-rpc url of the selected chain is used)

```ts
import { getConfig, createContext } from "@vlayer/sdk/config";

const config = getConfig();

const { chain, ethClient, account, proverUrl, confirmations } = await createContext(config);

const vlayer = createVlayerClient({
  url: proverUrl,
});
```

## Proving

In order to start proving, we will need to provide:
- `address` - an address of prover contract
- `proverAbi` - abi of prover contract
- `functionName` - name of prover contract function to call
- `args` - an array of arguments to `functionName` prover contract function 
- `chainId` - id of the chain in whose context the prover contract call shall be executed

```ts
const hash = await vlayer.prove({
  address: '0x70997970c51812dc3a010c7d01b50e0d17dc79c8',
  proverAbi: proverSpec.abi,
  functionName: 'main',
  args: ['0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045', 123],
  chain: chain.id,
});
```

### Waiting for result

Wait for the proving to be finished, and then retrieve the result along with Proof.

```ts
const result = await vlayer.waitForProvingResult({ hash });
```

By default, the `waitForProvingResult` function polls the server for a proof for 2 minutes. This is achieved through 120 retries with a polling interval of 1 second.
You can customize this behavior by specifying the following optional parameters:
- `numberOfRetries`: The total number of polling attempts.
- `sleepDuration`: The delay (in ms) between each polling attempt.
For example, if you want to extend the polling duration to 180 seconds with a 2-second delay between attempts, you can configure it as follows:

```ts
const provingResult = await vlayer.waitForProvingResult({
  numberOfRetries: 90,  // Total retries (180s / 2)
  sleepDuration: 2000,  // 2s interval between retries
});
```

## On-Chain verification

Once the proving result is obtained, one may call the verifier contract to validate the proof. Below is an example using the [viem](https://viem.sh/docs/contracts/write-contract/) library's `writeContract` function:

```ts
const txHash = await ethClient.writeContract({
  address: verifierAddr,
  abi: verifierSpec.abi,
  functionName: "verify",
  args: provingResult,
  chain,
  account,
});
```
