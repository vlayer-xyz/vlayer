import SimpleWebProof from "../components/SimpleWebProof";
import { WagmiProvider } from "wagmi";
import { ProofProvider } from "@vlayer/react";
import { http, createConfig } from "wagmi";
import { optimismSepolia, anvil } from "wagmi/chains";
import { metaMask } from "wagmi/connectors";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
const queryClient = new QueryClient();

export const config = createConfig({
  chains:
    import.meta.env.VITE_CHAIN_NAME === "anvil" ? [anvil] : [optimismSepolia],
  connectors: [metaMask()],
  transports: {
    [optimismSepolia.id]: http(),
    [anvil.id]: http(),
  },
});

const SimpleWebProofApp = () => {
  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        <ProofProvider
          config={{
            proverUrl: import.meta.env.VITE_PROVER_URL,
          }}
        >
          <SimpleWebProof />
        </ProofProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
};

export default SimpleWebProofApp;
