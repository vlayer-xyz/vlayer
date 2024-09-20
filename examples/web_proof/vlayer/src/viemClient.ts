import { createTestClient, http, publicActions, walletActions } from "viem";
import { foundry } from "viem/chains";

export const client = createTestClient({
  chain: foundry,
  mode: "anvil",
  transport: http(),
})
  .extend(publicActions)
  .extend(walletActions);
