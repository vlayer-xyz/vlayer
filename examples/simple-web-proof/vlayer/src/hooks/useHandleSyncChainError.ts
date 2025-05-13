import { ChainSwitchError } from "@vlayer/react";

import { ChainNotSupportedError } from "@vlayer/react";

import { P } from "ts-pattern";

import { match } from "ts-pattern";
import { AppError } from "../errors";
import { MissingChainError } from "@vlayer/react";
import { useEffect } from "react";
export const useHandleSyncChainError = (syncChainError: Error | null) => {
  useEffect(() => {
    if (syncChainError) {
      match(syncChainError)
        .with(P.instanceOf(MissingChainError), () => {
          throw new AppError("Missing chain", syncChainError.message);
        })
        .with(P.instanceOf(ChainNotSupportedError), () => {
          throw new AppError("Chain not supported", syncChainError.message);
        })
        .with(P.instanceOf(ChainSwitchError), () => {
          throw new AppError("Chain switch error", syncChainError.message);
        })
        .exhaustive();
    }
  }, [syncChainError?.name]);
};
