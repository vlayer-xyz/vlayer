import { useState } from "react";
import { useChainId } from "wagmi";
import {
  type Abi,
  type AbiStateMutability,
  type ContractFunctionArgs,
  type ContractFunctionName,
} from "viem";
import { type BrandedHash, type ProveArgs } from "@vlayer/sdk";
import { useProofContext } from "../context";

export enum ProverStatus {
  Idle = "Idle",
  Pending = "Pending",
  Ready = "Ready",
  Error = "Error",
}

export const useCallProver = (
  proveArgs: Omit<
    ProveArgs<Abi, ContractFunctionName<Abi>>,
    "chainId" | "args"
  >,
) => {
  // read vlayer client from context
  const { vlayerClient } = useProofContext();
  // read chainId from wagmi
  const chainId = useChainId();

  // state
  const [status, setStatus] = useState<ProverStatus>(ProverStatus.Idle);
  const [error, setError] = useState<Error | null>(null);
  const [hash, setHash] = useState<BrandedHash<Abi, string>>({
    hash: "",
  } as BrandedHash<Abi, string>);

  const callProver = async (
    args: ContractFunctionArgs<Abi, AbiStateMutability, string>,
  ) => {
    setStatus(ProverStatus.Pending);
    try {
      const hash = await vlayerClient.prove({
        ...proveArgs,
        args,
        chainId,
      });
      setHash(hash);
      setStatus(ProverStatus.Ready);
    } catch (e) {
      setError(e as Error);
      setStatus(ProverStatus.Error);
    }
  };

  return {
    callProver,
    status,
    error,
    data: hash,
    isIdle: status === ProverStatus.Idle,
    isPending: status === ProverStatus.Pending,
    isReady: status === ProverStatus.Ready,
    isError: status === ProverStatus.Error,
  };
};
