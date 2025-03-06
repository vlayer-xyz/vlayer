import { steps } from "./utils/steps";
import { createConfig, createConnector, http, WagmiProvider } from "wagmi";
import { ProofProvider } from "@vlayer/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Routes, Route } from "react-router";
import { WagmiAdapter } from "@reown/appkit-adapter-wagmi";
import { createAppKit } from "@reown/appkit/react";
import { optimismSepolia, anvil, Chain } from "@reown/appkit/networks";
import { Layout } from "./components/layout/Layout";
import { privateKeyToAccount } from "viem/accounts";

const queryClient = new QueryClient();

export enum ClientAuthMode {
  ENV_PRIVATE_KEY = "envPrivateKey",
  WALLET = "wallet",
}

const projectId = `0716afdbbb2cc3df69721a879b92ad5b`;
const chain =
  import.meta.env.VITE_CHAIN_NAME === "anvil" ? anvil : optimismSepolia;
const chains: [Chain, ...Chain[]] = [chain];
const networks = chains;

const getAddressFromPrivateKey = () => {
  let address = "";
  const envPrivateKey = import.meta.env.VITE_PRIVATE_KEY;
  if (!envPrivateKey) {
    throw new Error("No private key found");
  } else {
    address = privateKeyToAccount(envPrivateKey as "0x").address;
  }
  return address as "0x";
};

const mockConnector = createConnector((config) => ({
  ...config,
  id: "mock-connector",
  name: "Mock Connector",
  type: "mock",
  connect: async () => ({
    accounts: [getAddressFromPrivateKey()],
    chainId: chain.id,
  }),
  disconnect: async () => {},
  getAccounts: async () => [getAddressFromPrivateKey()],
  getChainId: async () => chain.id,
  getProvider: async () => ({}),
  isAuthorized: async () => true,
  address: getAddressFromPrivateKey(),
  onAccountsChanged: () => {},
  onChainChanged: () => {},
  onDisconnect: () => {},
}));

const config = () => {
  const authMode = import.meta.env.VITE_CLIENT_AUTH_MODE;

  switch (authMode) {
    case ClientAuthMode.ENV_PRIVATE_KEY: {
      return createConfig({
        connectors: [mockConnector],
        chains,
        transports: {
          [anvil.id]: http(),
        },
      });
    }
    case ClientAuthMode.WALLET: {
      return wagmiAdapter.wagmiConfig;
    }
    default: {
      throw new Error("Invalid VITE_CLIENT_AUTH_MODE: " + authMode);
    }
  }
};

const wagmiAdapter = new WagmiAdapter({
  projectId,
  chains,
  networks,
});

createAppKit({
  adapters: [wagmiAdapter],
  projectId,
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
      <WagmiProvider config={config()}>
        <QueryClientProvider client={queryClient}>
          <ProofProvider
            config={{
              proverUrl: import.meta.env.VITE_PROVER_URL,
              wsProxyUrl: import.meta.env.VITE_WS_PROXY_URL,
              notaryUrl: import.meta.env.VITE_NOTARY_URL,
            }}
          >
            <BrowserRouter>
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
            </BrowserRouter>
          </ProofProvider>
        </QueryClientProvider>
      </WagmiProvider>
    </div>
  );
};

export default App;
