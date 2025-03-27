import { http, createConfig } from "wagmi";
import { optimismSepolia, anvil } from "wagmi/chains";
import { metaMask } from "wagmi/connectors";
import { useEnvPrivateKey } from "../../shared/lib/clientAuthMode";
import { mockConnector } from "../../shared/lib/mockConnector";

const chain =
  import.meta.env.VITE_CHAIN_NAME === "anvil" ? anvil : optimismSepolia;

const wagmiConfig = useEnvPrivateKey()
  ? createConfig({
      connectors: [mockConnector(chain)],
      chains: [chain],
      transports: {
        [anvil.id]: http(),
        [optimismSepolia.id]: http(),
      },
    })
  : createConfig({
      chains:
        import.meta.env.VITE_CHAIN_NAME === "anvil"
          ? [anvil]
          : [optimismSepolia],
      connectors: [metaMask()],
      transports: {
        [anvil.id]: http(),
        [optimismSepolia.id]: http(),
      },
    });

const proverConfig = {
  proverUrl: import.meta.env.VITE_PROVER_URL,
  token: import.meta.env.VITE_VLAYER_API_TOKEN,
};

export { wagmiConfig, proverConfig };
