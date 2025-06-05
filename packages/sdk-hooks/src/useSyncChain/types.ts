import type { Chain } from "viem";

export type ChainAction =
  | { type: "NO_CHAIN" }
  | { type: "CHAIN_NOT_SUPPORTED"; payload: string }
  | { type: "CHAIN_SWITCHED"; payload: Chain }
  | { type: "CHAIN_IN_SYNC"; payload: Chain }
  | { type: "CHAIN_SWITCH_ERROR"; payload: string };

export type ChainState = {
  chain: Chain | null;
  error: Error | null;
  switched: boolean;
};
