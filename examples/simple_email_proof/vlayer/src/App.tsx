import { WagmiProvider, http, createConfig } from "wagmi";
import { ProofProvider } from "@vlayer/react";
import EmlUploadForm from "./containers/EmlUploadForm";

import { optimismSepolia, foundry } from "wagmi/chains";
import { metaMask } from "wagmi/connectors";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const wagmiConfig = createConfig({
  chains:
    import.meta.env.VITE_CHAIN_NAME === "anvil" ? [foundry] : [optimismSepolia],
  connectors: [metaMask()],
  transports: {
    [foundry.id]: http(),
    [optimismSepolia.id]: http(),
  },
});

const queryClient = new QueryClient();

function App() {
  return (
    <WagmiProvider config={wagmiConfig}>
      <QueryClientProvider client={queryClient}>
        <ProofProvider>
          <EmlUploadForm />
        </ProofProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
}

export default App;
