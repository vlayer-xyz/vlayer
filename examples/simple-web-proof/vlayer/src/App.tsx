import {
  WelcomeScreen,
  ConnectWalletStep,
  ProveStep,
  MintingContainer,
  SuccessContainer,
} from "./components";
// import { ExtensionCheck } from "./ExtensionCheck";
import { WagmiProvider } from "wagmi";
import { ProofProvider } from "@vlayer/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Routes, Route } from "react-router";
import { WagmiAdapter } from "@reown/appkit-adapter-wagmi";
import { createAppKit } from "@reown/appkit/react";
import { optimismSepolia, anvil } from "@reown/appkit/networks";
import { Layout } from "./components/layout/Layout";
import { Outlet } from "react-router";
const queryClient = new QueryClient();

const wagmiAdapter = new WagmiAdapter({
  projectId: `0716afdbbb2cc3df69721a879b92ad5b`,
  networks: [optimismSepolia, anvil],
  chains:
    import.meta.env.VITE_CHAIN_NAME === "anvil" ? [anvil] : [optimismSepolia],
});

createAppKit({
  adapters: [wagmiAdapter],
  projectId: `0716afdbbb2cc3df69721a879b92ad5b`,
  networks: [optimismSepolia, anvil],
  defaultNetwork:
    import.meta.env.VITE_CHAIN_NAME === "anvil" ? anvil : optimismSepolia,
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
            }}
          >
            <BrowserRouter>
              <Routes>
                <Route path="/" element={<Outlet />}>
                  <Route index element={<WelcomeScreen />} />
                  {/* <Route path="/extension-check" element={<ExtensionCheck />} /> */}

                  <Route path="/proof" element={<Layout />}>
                    <Route
                      path="connect-wallet"
                      element={<ConnectWalletStep />}
                    />
                    <Route
                      path="start-proving"
                      element={<ProvingContainer />}
                    />
                    <Route path="minting" element={<MintingContainer />} />
                    <Route path="success" element={<SuccessContainer />} />
                  </Route>
                </Route>
              </Routes>
            </BrowserRouter>
          </ProofProvider>
        </QueryClientProvider>
      </WagmiProvider>
    </div>
  );
};

export default App;
