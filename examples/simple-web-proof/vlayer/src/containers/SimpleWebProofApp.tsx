import SimpleWebProof from "../components/SimpleWebProof";
import { WelcomeScreen } from "../components/WelcomeScreen";
import { ConnectWallet } from "../components/ConnectWallet";
import { StartProving } from "../components/StartProving";
import { Minting } from "../components/Minting";
import { Success } from "../components/Success";
import { WagmiProvider } from "wagmi";
import { ProofProvider } from "@vlayer/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Route, Routes } from "react-router";
import { WagmiAdapter } from "@reown/appkit-adapter-wagmi";
import { createAppKit } from "@reown/appkit/react";
import { optimismSepolia, anvil } from "@reown/appkit/networks";

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
    name: "appkit-example",
    description: "AppKit Example",
    url: "https://appkitexampleapp.com", // origin must match your domain & subdomain
    icons: ["https://avatars.githubusercontent.com/u/179229932"]
  },
});

// export const config = createConfig({
//   chains:
//     import.meta.env.VITE_CHAIN_NAME === "anvil" ? [anvil] : [optimismSepolia],
//   connectors: [metaMask()],
//   transports: {
//     [optimismSepolia.id]: http(),
//     [anvil.id]: http(),
//   },
// });

const SimpleWebProofApp = () => {
  return (
    <WagmiProvider config={wagmiAdapter.wagmiConfig}>
      <QueryClientProvider client={queryClient}>
        <ProofProvider
          config={{
            proverUrl: import.meta.env.VITE_PROVER_URL,
          }}
        >
          <BrowserRouter>
            <Routes>
              <Route path="/" element={<WelcomeScreen />} />
              <Route path="/connect-wallet" element={<ConnectWallet />} />
              <Route path="/start-proving" element={<StartProving />} />
              <Route path="/minting" element={<Minting />} />
              <Route path="/success" element={<Success />} />
              <Route path="/simple" element={<SimpleWebProof />} />
            </Routes>
          </BrowserRouter>
        </ProofProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
};

export default SimpleWebProofApp;
