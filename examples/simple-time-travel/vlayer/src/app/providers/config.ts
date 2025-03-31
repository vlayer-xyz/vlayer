import { http, createConfig } from "wagmi";
import { optimismSepolia, anvil } from "wagmi/chains";

const wagmiConfig = createConfig({
  chains:
    import.meta.env.VITE_CHAIN_NAME === "anvil" ? [anvil] : [optimismSepolia],
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
