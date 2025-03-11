import { steps } from "./utils/steps";
import { config } from "./utils/wagmiProviderConfig";
import { WagmiProvider } from "wagmi";
import { ProofProvider } from "@vlayer/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Routes, Route } from "react-router";
import { Layout } from "./components/layout/Layout";

const queryClient = new QueryClient();

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
