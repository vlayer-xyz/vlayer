import { http, createConfig } from "wagmi";
import { optimismSepolia, anvil } from "wagmi/chains";
import { metaMask } from "wagmi/connectors";
import { useEnvPrivateKey } from "../../shared/lib/clientAuthMode";
import { mockConnector } from "../../shared/lib/mockConnector";

let wagmiConfig;

if (useEnvPrivateKey()) {
  wagmiConfig = createConfig({
    connectors: [mockConnector(anvil)],
    chains: [anvil],
    transports: {
      [anvil.id]: http(),
    },
  });
} else {
  wagmiConfig = createConfig({
    chains:
      import.meta.env.VITE_CHAIN_NAME === "anvil" ? [anvil] : [optimismSepolia],
    connectors: [metaMask()],
    transports: {
      [anvil.id]: http(),
      [optimismSepolia.id]: http(),
    },
  });
}

const proverConfig = {
  proverUrl: import.meta.env.VITE_PROVER_URL,
  token: import.meta.env.VITE_VLAYER_API_TOKEN,
};

export { wagmiConfig, proverConfig };
