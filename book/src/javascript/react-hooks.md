# React Hooks for vlayer
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
import { sepolia, baseSepolia,optimismSepolia, foundry } from "wagmi/chains";
import { metaMask } from "wagmi/connectors";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ProofProvider } from "@vlayer/react";

const wagmiConfig = createConfig({
  chains: [sepolia, baseSepolia, optimismSepolia, foundry],
  connectors: [metaMask()],
  transports: {
    [sepolia.id]: http(),
    [baseSepolia.id]: http(),
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

The `ProofProvider` component in vlayer is pre-configured for the [testnet environment](/advanced/dev-and-production.html#public-testnet-services) by default, requiring no additional props for basic usage:

```javascript
<ProofProvider>
    {/* Application components go here */}
</ProofProvider>
```

### Using the `config` Prop

The `ProofProvider` also accepts an optional `config` prop, enabling you to select the desired `env`. Based on the chosen environment, the provider is automatically configured with the [default and pre-configured URLs](/advanced/dev-and-production.html#public-testnet-services) necessary to access vlayer network services:

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
In addition to selecting an environment, the `config` prop allows you to specify custom URLs for [vlayer network services](/advanced/dev-and-production.html). These include services like `proverUrl`, `notaryUrl`, and `wsProxyUrl`:

```javascript
<ProofProvider
  config={{
    proverUrl: "https://test-prover.vlayer.xyz",
    notaryUrl: "https://test-notary.vlayer.xyz",
    wsProxyUrl: "wss://test-ws-proxy.vlayer.xyz",
  }}
>
    {/* Application components go here */}
</ProofProvider>
```

## `useCallProver`
The `useCallProver` hook is used to interact with the vlayer prover by initiating a proving process with specified inputs.


### Example usage
The `callProver` function starts the proving process. Proving is an asynchronous operation, and the result (`data`) contains a hash that can be used to track the status or [retrieve the final proof](/javascript/react-hooks.html#usewaitforprovingresult).

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
- `status`: Overall status of the proving process (`idle`, `pending`, `ready`, or `error`).
- `isIdle`: Indicates no proving request has been initiated.
- `isPending`: Indicates the proving process is ongoing.
- `isReady`: Indicates the proving process is complete.
- `isError`: Indicates an error occurred.

## `useWaitForProvingResult`
The `useWaitForProvingResult` hook waits for a proving process to complete and retrieves the resulting proof.

### Example usage
Pass the proof hash to the hook to monitor the proving process and retrieve the proof (`data`) when it is ready. If no hash (`null`) is provided, no request is sent to the prover.

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
- `isIdle`: Indicates the hook is waiting for a hash to monitor.
- `isPending`: Indicates the proof generation is ongoing.
- `isReady`: Indicates the proof is available.
- `isError`: Indicates an error occurred during proof retrieval.

> 💡 **Try it Now**
> 
> To see vlayer React hooks in action, run the following command in your terminal:
> 
> ```bash
> vlayer init --template simple-email-proof
> ```
> 
> This command will download create a new project. Check out the `vlayer/src/components/EmlForm.tsx` file to see how vlayer React hooks are used.