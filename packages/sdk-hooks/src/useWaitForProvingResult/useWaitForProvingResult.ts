import type { BrandedHash } from "@vlayer/sdk";
import { useState } from "react";
import { useProofContext } from "../context";
import type { Abi } from "viem";

export enum WaitForProvingResultStatus {
  Idle = "Idle",
  Pending = "Pending",
  Ready = "Ready",
  Error = "Error",
}

export const useWaitForProvingResult = (hash: BrandedHash<Abi, string>) => {
  const { vlayerClient } = useProofContext();
  const [status, setStatus] = useState<WaitForProvingResultStatus>(
    WaitForProvingResultStatus.Idle,
  );
  const [error, setError] = useState<Error | null>(null);
  const [result, setResult] = useState<unknown>(null);
  const waitForProvingResult = async () => {
    setStatus(WaitForProvingResultStatus.Pending);
    try {
      const result = await vlayerClient.waitForProvingResult({ hash });
      setStatus(WaitForProvingResultStatus.Ready);
      setResult(result);
    } catch (e) {
      setError(e as Error);
      setStatus(WaitForProvingResultStatus.Error);
    }
  };

  return {
    waitForProvingResult,
    status,
    error,
    isIdle: status === WaitForProvingResultStatus.Idle,
    isPending: status === WaitForProvingResultStatus.Pending,
    isReady: status === WaitForProvingResultStatus.Ready,
    isError: status === WaitForProvingResultStatus.Error,
    data: result,
  };
};
