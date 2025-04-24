# React Hooks for vlayer

[@vlayer/react](https://www.npmjs.com/package/@vlayer/react) is a library of React hooks for interacting with the vlayer.

These hooks provide functions that help manage state and side effects in React components, aligning with React's functional programming paradigm and style of [wagmi](https://wagmi.sh/docs/getting-started) hooks.

## Prerequisites
The following libraries are required to use `@vlayer/react`:
- [React](https://reactjs.org/docs/getting-started.html): A library for building user interfaces.
- [Wagmi](https://wagmi.sh/docs/getting-started): A library of React hooks for Ethereum.
- [TanStack Query](https://tanstack.com/query/latest): A library for efficient data fetching and caching.

Add them to your project if they are not already present: 
{{#tabs }}
{{#tab name="yarn" }}
```sh
yarn add react react-dom wagmi @tanstack/react-query
```
{{#endtab }}
{{#tab name="npm" }}
```sh
npm install react react-dom wagmi @tanstack/react-query
```
{{#endtab }}
{{#tab name="pnpm" }}
```sh
pnpm install react react-dom wagmi @tanstack/react-query
```
{{#endtab }}
{{#tab name="bun" }}
```sh
bun add react react-dom wagmi @tanstack/react-query
```
{{#endtab }}
{{#endtabs }}

## Installation
Install the `@vlayer/react` library using preferred package manager:

{{#tabs }}
{{#tab name="yarn" }}
```sh
yarn add @vlayer/react
```
{{#endtab }}
{{#tab name="npm" }}
```sh
npm install @vlayer/react
```
{{#endtab }}
{{#tab name="pnpm" }}
```sh
pnpm install @vlayer/react
```
{{#endtab }}
{{#tab name="bun" }}
```sh
bun add @vlayer/react
```
{{#endtab }}
{{#endtabs }}

## Context Providers
Wrap the application with the required [React Context Providers](https://react.dev/learn/passing-data-deeply-with-context) and configure the desired connectors and chains to enable `@vlayer/react` hooks.

```javascript
import { WagmiProvider, http, createConfig } from "wagmi";
import { baseSepolia, sepolia,optimismSepolia, foundry } from "wagmi/chains";
import { metaMask } from "wagmi/connectors";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ProofProvider } from "@vlayer/react";

const wagmiConfig = createConfig({
  chains: [baseSepolia, sepolia, optimismSepolia, foundry],
  connectors: [metaMask()],
  transports: {
    [baseSepolia.id]: http(),
    [sepolia.id]: http(),
    [optimismSepolia.id]: http(),
    [foundry.id]: http(),
  },
});

const queryClient = new QueryClient();

function App() {
  return (
    <WagmiProvider config={wagmiConfig}>
      <QueryClientProvider client={queryClient}>
        <ProofProvider>
          {/* Application components go here */}
        </ProofProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
}

export default App;
```

Context providers facilitate the sharing of application state (e.g., connected wallet, selected chain) across components. Once the setup is complete, components wrapped within the `WagmiProvider`, `QueryClientProvider`, and `ProofProvider` can use the vlayer hooks.

Your section on configuring `ProofProvider` is well-structured and clear. Here are some suggestions to improve grammar, style, and clarity while maintaining the current structure:


## Configuring `ProofProvider`

The `ProofProvider` component in vlayer is pre-configured for the [testnet environment](/getting-started/dev-and-production.html#public-testnet-services) by default, requiring no additional props for basic usage:

```javascript
<ProofProvider>
    {/* Application components go here */}
</ProofProvider>
```

### Using the `config` Prop

The `ProofProvider` also accepts an optional `config` prop, enabling you to select the desired `env`. Based on the chosen environment, the provider is automatically configured with the [default and pre-configured URLs](/getting-started/dev-and-production.html#public-testnet-services) necessary to access vlayer network services:

```javascript
<ProofProvider
  config={{
    env: "dev|testnet|prod", // Specify the environment
  }}
>
    {/* Application components go here */}
</ProofProvider>
```

### Customizing Service URLs
In addition to selecting an environment, the `config` prop allows you to specify custom URLs for [vlayer network services](/getting-started/dev-and-production.html). These include services like `proverUrl`, `notaryUrl`, and `wsProxyUrl`:

```javascript
<ProofProvider
  config={{
    proverUrl: "https://stable-fake-prover.vlayer.xyz",
    notaryUrl: "https://test-notary.vlayer.xyz",
    wsProxyUrl: "wss://test-wsproxy.vlayer.xyz",
  }}
>
    {/* Application components go here */}
</ProofProvider>
```

## `useCallProver`
The `useCallProver` hook is used to interact with the vlayer prover by initiating a proving process with specified inputs.


### Example usage
The `callProver` function initiates the proving process. Proving is an asynchronous operation, and the result (`data`) contains a hash that can be used to track the status or [retrieve the final proof](/javascript/react-hooks.html#usewaitforprovingresult).

```javascript
import { useCallProver } from "@vlayer/react";

const ExampleComponent = () => {
  const { 
    callProver, 
    data, 
    status, 
    error, 
    isIdle, 
    isPending, 
    isReady, 
    isError 
  } = useCallProver({
    address: proverAddress,     // Address of the prover contract
    proverAbi: proverSpec.abi,  // ABI of the prover
    functionName: "main",       // Function to invoke in the prover
  });

  return (
    <button onClick={() => callProver([...args])}>
      Prove
    </button>
  );
}
```

The `callProver` function has to be invoked with the required arguments by the prover contract function.

Besides proof hash, the hook returns variables to monitor the request and update the UI:
- `status`: Overall status of the initial call to the prover (`idle`, `pending`, `ready`, or `error`).
- `isIdle`: Indicates that no prover call has been initiated.
- `isPending`: Indicates the waiting for proving hash is ongoing.
- `isReady`: Indicates the proving hash is available.
- `isError`: Indicates an error occurred.
- `error`: Contains the error message if an error occurred.

## `useWaitForProvingResult`
The `useWaitForProvingResult` hook waits for a proving process to complete and retrieves the resulting proof.

### Example usage
Pass the proof hash to the hook to monitor the proving process and retrieve the proof (`data`) when it is ready. If no hash (`null`) is provided, no request is sent to the prover.

Proof computation is an asynchronous operation, and depending on the complexity of the proof, it may take a few seconds to complete. Proof is `null` until the computation is complete.

```javascript
import { useWaitForProvingResult, useCallProver } from "@vlayer/react";

const ExampleComponent = () => {
  const { callProver, data: proofHash } = useCallProver({
    address: proverAddress,     // Address of the prover contract
    proverAbi: proverSpec.abi,  // ABI of the prover
    functionName: "main",       // Function to invoke in the prover
  });

  const { 
    data, 
    error, 
    status, 
    isIdle, 
    isPending, 
    isReady, 
    isError 
  } = useWaitForProvingResult(proofHash);

  return (
    <button onClick={() => callProver([...args])}>
      Prove
    </button>
  );
}

```

The hook provides additional properties for tracking progress and managing UI updates:
- `status`: Indicates the status of the proving result (`idle`, `pending`, `ready`, or `error`).
- `isIdle`: Indicates the hook is not triggered.
- `isPending`: Indicates the proof computation is ongoing.
- `isReady`: Indicates the final proof is available.
- `isError`: Indicates an error occurred during proving.
- `error`: Contains the error message returned by the prover
> 💡 **Try it Now**
> 
> To see vlayer React hooks in action, run the following command in your terminal:
> 
> ```bash
> vlayer init --template simple-email-proof
> ```
> 
> This command will download create a new project. Check out the `vlayer/src/components/EmlForm.tsx` file to see how vlayer React hooks are used.