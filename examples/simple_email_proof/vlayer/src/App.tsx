import { WagmiProvider, http, createConfig } from "wagmi";
import { ProofProvider } from "@vlayer/react";
import EmlUploadForm from "./containers/EmlUploadForm";

import { optimismSepolia, foundry } from "wagmi/chains";

export const wagmiConfig = createConfig({
  chains: [optimismSepolia, foundry],
  transports: {
    [optimismSepolia.id]: http(),
    [foundry.id]: http(),
  },
});

function App() {
  return (
    <WagmiProvider config={wagmiConfig}>
      <ProofProvider>
        <EmlUploadForm />
      </ProofProvider>
    </WagmiProvider>
  );
}

export default App;
