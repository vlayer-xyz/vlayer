import { Chain } from "wagmi/chains";
import { createAppKit } from "@reown/appkit/react";
import { WagmiAdapter } from "@reown/appkit-adapter-wagmi";
import { getChainSpecs } from "@vlayer/sdk";

const appKitProjectId = `0716afdbbb2cc3df69721a879b92ad5b`;
let chain = null;

try {
  chain = getChainSpecs(import.meta.env.VITE_CHAIN_NAME);
} catch {
  // In case of wrong chain name in env, we set chain variable to whatever.
  // Thanks to this, the app does not crash here, but later with a proper error handling.
  console.error("Wrong chain name in env: ", import.meta.env.VITE_CHAIN_NAME);
  chain = {
    id: "wrongChain",
    name: "Wrong chain",
    nativeCurrency: {},
    rpcUrls: { default: { http: [] } },
  } as unknown as Chain;
}
const chains: [Chain, ...Chain[]] = [chain];
const networks = chains;

const wagmiAdapter = new WagmiAdapter({
  projectId: appKitProjectId,
  chains,
  networks,
});

createAppKit({
  adapters: [wagmiAdapter],
  projectId: appKitProjectId,
  networks,
  defaultNetwork: chain,
  metadata: {
    name: "vlayer-email-proof-example",
    description: "vlayer Email Proof Example",
    url: "https://vlayer.xyz",
    icons: ["https://avatars.githubusercontent.com/u/179229932"],
  },
  themeVariables: {
    "--w3m-color-mix": "#551fbc",
    "--w3m-color-mix-strength": 40,
  },
});

const proverConfig = {
  proverUrl: import.meta.env.VITE_PROVER_URL,
  token: import.meta.env.VITE_VLAYER_API_TOKEN,
};

const { wagmiConfig } = wagmiAdapter;

export { wagmiConfig, proverConfig };
