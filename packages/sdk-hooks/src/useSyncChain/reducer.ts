import type { ChainAction } from "./types";
import { match } from "ts-pattern";
import type { ChainState } from "./types";
import {
  MissingConfigChainError,
  ChainNotSupportedError,
  ChainSwitchError,
} from "./errors";

export const reducer = (state: ChainState, action: ChainAction): ChainState => {
  return match(action)
    .with({ type: "NO_CHAIN" }, () => ({
      ...state,
      chain: null,
      error: new MissingConfigChainError(),
      switched: false,
    }))
    .with({ type: "CHAIN_NOT_SUPPORTED" }, ({ payload }) => ({
      ...state,
      chain: null,
      error: new ChainNotSupportedError(payload),
      switched: false,
    }))
    .with({ type: "CHAIN_SWITCHED" }, ({ payload }) => ({
      ...state,
      chain: payload,
      error: null,
      switched: true,
    }))
    .with({ type: "CHAIN_IN_SYNC" }, ({ payload }) => ({
      ...state,
      chain: payload,
      error: null,
      switched: false,
    }))
    .with({ type: "CHAIN_SWITCH_ERROR" }, ({ payload }) => ({
      ...state,
      chain: null,
      error: new ChainSwitchError(payload),
      switched: false,
    }))
    .otherwise(() => state);
};
