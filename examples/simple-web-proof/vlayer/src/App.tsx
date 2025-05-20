import { steps } from "./utils/steps";
import { WagmiProvider } from "wagmi";
import { ProofProvider } from "@vlayer/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Routes, Route } from "react-router";
import { Layout } from "./components/layout/Layout";
import { createAppKit } from "@reown/appkit/react";
import { WagmiAdapter } from "@reown/appkit-adapter-wagmi";
import { Chain } from "viem";
import { ErrorBoundary } from "react-error-boundary";
import { AppErrorBoundaryComponent } from "./components/layout/ErrorBoundary";
import { getChainSpecs } from "@vlayer/sdk";

const queryClient = new QueryClient();
const appKitProjectId = `0716afdbbb2cc3df69721a879b92ad5b`;
let chain = null;

try {
  chain = getChainSpecs(import.meta.env.VITE_CHAIN_NAME);
} catch {
  // In case of wrong chain name in env, we set chain variable to whatever.
  // Thanks to this, the app does not crash here, but later with a proper error handling.
  console.error("Wrong chain name in env: ", import.meta.env.VITE_CHAIN_NAME);
  chain = {
    id: "wrongChain",
    name: "Wrong chain",
    nativeCurrency: {},
    rpcUrls: { default: { http: [] } },
  } as unknown as Chain;
}
const chains: [Chain, ...Chain[]] = [chain];
const networks = chains;

const wagmiAdapter = new WagmiAdapter({
  projectId: appKitProjectId,
  chains,
  networks,
});

createAppKit({
  adapters: [wagmiAdapter],
  projectId: appKitProjectId,
  networks,
  defaultNetwork: chain,
  metadata: {
    name: "vlayer-web-proof-example",
    description: "vlayer Web Proof Example",
    url: "https://vlayer.xyz",
    icons: ["https://avatars.githubusercontent.com/u/179229932"],
  },
  themeVariables: {
    "--w3m-color-mix": "#551fbc",
    "--w3m-color-mix-strength": 40,
  },
});

const App = () => {
  return (
    <div id="app">
      <WagmiProvider config={wagmiAdapter.wagmiConfig}>
        <QueryClientProvider client={queryClient}>
          <ProofProvider
            config={{
              proverUrl: import.meta.env.VITE_PROVER_URL,
              wsProxyUrl: import.meta.env.VITE_WS_PROXY_URL,
              notaryUrl: import.meta.env.VITE_NOTARY_URL,
              token: import.meta.env.VITE_VLAYER_API_TOKEN,
            }}
          >
            <BrowserRouter>
              <ErrorBoundary FallbackComponent={AppErrorBoundaryComponent}>
                <Routes>
                  <Route path="/" element={<Layout />}>
                    {steps.map((step) => (
                      <Route
                        key={step.path}
                        path={step.path}
                        element={<step.component />}
                      />
                    ))}
                  </Route>
                </Routes>
              </ErrorBoundary>
            </BrowserRouter>
          </ProofProvider>
        </QueryClientProvider>
      </WagmiProvider>
    </div>
  );
};

export default App;
