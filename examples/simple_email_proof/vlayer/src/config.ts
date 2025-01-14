import { http, createConfig } from "wagmi";
import { optimismSepolia, foundry } from "wagmi/chains";
import { metaMask } from "wagmi/connectors";

export const wagmiConfig = createConfig({
  chains:
    import.meta.env.VITE_CHAIN_NAME === "anvil" ? [foundry] : [optimismSepolia],
  connectors: [metaMask()],
  transports: {
    [foundry.id]: http(),
    [optimismSepolia.id]: http(),
  },
});

export const proverConfig = {
  proverUrl: import.meta.env.VITE_PROVER_URL,
};
