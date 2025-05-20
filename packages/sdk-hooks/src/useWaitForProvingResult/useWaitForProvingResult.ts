import { type BrandedHash, type Metrics } from "@vlayer/sdk";
import { useState, useEffect } from "react";
import { useProofContext } from "../context";
import type { Abi } from "viem";

export enum WaitForProvingResultStatus {
  Idle = "Idle",
  Pending = "Pending",
  Ready = "Ready",
  Error = "Error",
}

export const useWaitForProvingResult = (
  hash: BrandedHash<Abi, string> | null,
) => {
  const { vlayerClient } = useProofContext();
  const [status, setStatus] = useState<WaitForProvingResultStatus>(
    WaitForProvingResultStatus.Idle,
  );
  const [error, setError] = useState<Error | null>(null);
  const [result, setResult] = useState<unknown>(null);
  const [metrics, setMetrics] = useState<Metrics | null>(null);
  useEffect(() => {
    if (!hash) {
      return;
    }
    setStatus(WaitForProvingResultStatus.Pending);
    vlayerClient
      .waitForProvingResult({ hash })
      .then(({ proof, metrics }) => {
        setStatus(WaitForProvingResultStatus.Ready);
        setResult(proof);
        setMetrics(metrics);
      })
      .catch((e) => {
        setError(e as Error);
        setStatus(WaitForProvingResultStatus.Error);
      });
  }, [JSON.stringify(hash)]);

  return {
    status,
    error,
    isIdle: status === WaitForProvingResultStatus.Idle,
    isPending: status === WaitForProvingResultStatus.Pending,
    isReady: status === WaitForProvingResultStatus.Ready,
    isError: status === WaitForProvingResultStatus.Error,
    data: result,
    metrics,
  };
};
