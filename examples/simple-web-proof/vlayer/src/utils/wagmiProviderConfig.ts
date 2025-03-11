import { createAppKit } from "@reown/appkit";
import { WagmiAdapter } from "@reown/appkit-adapter-wagmi";
import { Chain, http } from "viem";
import { anvil, optimismSepolia } from "viem/chains";
import { createConfig } from "wagmi";
import { mockConnector } from "./mockConnector";
import { useEnvPrivateKey } from "./clientAuthMode";

export enum ClientAuthMode {
  ENV_PRIVATE_KEY = "envPrivateKey",
  WALLET = "wallet",
}

const appKitProjectId = `0716afdbbb2cc3df69721a879b92ad5b`;
const chain =
  import.meta.env.VITE_CHAIN_NAME === "anvil" ? anvil : optimismSepolia;
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
    name: "vlayer-web-proof-example",
    description: "vlayer Web Proof Example",
    url: "https://vlayer.xyz",
    icons: ["https://avatars.githubusercontent.com/u/179229932"],
  },
  themeVariables: {
    "--w3m-color-mix": "#551fbc",
    "--w3m-color-mix-strength": 40,
  },
});

export const config = () => {
  return useEnvPrivateKey()
    ? createConfig({
        connectors: [mockConnector(chain)],
        chains,
        transports: {
          [anvil.id]: http(),
        },
      })
    : wagmiAdapter.wagmiConfig;
};
