# Vanilla JS/TS
# JavaScript

Common way to interact with blockchain is to make calls and send transactions from JavaScript, most often form a web browser. vlayer provides developer friendly JavaScript/TypeScript api - vlayer sdk. It combines well with usual way of interacting with smart contracts. 

## Installation
To install vlayer SDK, run the following command in your JavaScript application

```sh
yarn add @vlayer/sdk
```

## vlayer client

A vlayer client is an interface to vlayer JSON-RPC API methods to trigger and follow the status of proving.

Initialize a client with default prover.

```ts
import { createVlayerClient } from '@vlayer/sdk'
 
const client = createVlayerClient();
```

Initialize a client with prover with specific url.

```ts
import { createVlayerClient } from '@vlayer/sdk'
 
const vlayerClient = createVlayerClient({
  url: 'localhost:3000'
})
```

## Encoding transaction
vlayer sdk is based on [viem](https://viem.sh/) - a TypeScript interface for Ethereum. If you used viem before, you will feel right at home. 

We will start with encoding a call to a prover contract, with viem:

```ts
import { encodeFunctionData } from 'viem'
import { proverAbi } from './abi'
 
const data = encodeFunctionData({
  abi: proverAbi,
  functionName: 'main',
  args: ['0xa5cc3c03994DB5b0d9A5eEdD10CabaB0813678AC']
})
```

## Proving

Now, we can request proving. We will need to provide:
- `to` - an address of prover contract
- `data` - encoded call to prover for the section above
- `chain` - a chain which will be used to settle a transaction

```ts
import { sepolia } from 'viem/chains'

const hash = vlayerClient.prove({
    to: '0x70997970c51812dc3a010c7d01b50e0d17dc79c8',
    data
    chain: sepolia
});
```

### Waiting for result
Wait for the proving to be finish, and then returns the result along with Proof.

```ts
const { proof, result } = await vlayerClient.waitForProvingResult({ hash });
```


