# Vanilla JS/TS
## JavaScript
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

The **vlayer client** provides an interface for interacting with the vlayer JSON-RPC API. It enables you to trigger and monitor proof statuses while offering convenient access to features such as [Web Proofs](/features/web.html) and [Email Proofs](/features/email.html).

### Initializing

You can initialize a client as shown below:

```ts
import { createVlayerClient } from "@vlayer/sdk";

const vlayer = createVlayerClient();
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
  chainId: chain.id,
});
```

### Waiting for result

Wait for the proving to be finished, and then retrieve the result along with Proof.

```ts
const result = await vlayer.waitForProvingResult({ hash });
```

By default, the `waitForProvingResult` function polls the server for a proof for 15 minutes. This is achieved through 900 retries with a polling interval of 1 second.
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

Once the proving result is obtained, one may call the verifier contract to validate the proof. Below is an example using the [viem](https://viem.sh/docs/contract/writeContract.html) library's `writeContract` function:

```ts
// Create client, see docs here: https://viem.sh/docs/clients/wallet
// const client = createWalletClient({...}); 

const txHash = await client.writeContract({
  address: verifierAddr,
  abi: verifierSpec.abi,
  functionName: "verify",
  args: provingResult,
  chain,
  account,
});
```
