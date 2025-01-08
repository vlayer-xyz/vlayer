import { WagmiProvider, http, createConfig } from "wagmi";
import { ProofProvider } from "@vlayer/react";
import EmlUploadForm from "./containers/EmlUploadForm";

import { optimismSepolia, foundry } from "wagmi/chains";
import { metaMask } from "wagmi/connectors";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const wagmiConfig = createConfig({
  chains: [optimismSepolia, foundry],
  connectors: [metaMask()],
  transports: {
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
          <EmlUploadForm />
        </ProofProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
}

export default App;
