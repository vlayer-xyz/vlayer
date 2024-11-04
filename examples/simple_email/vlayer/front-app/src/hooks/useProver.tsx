import { useMemo, useEffect, useState } from "react";
import { createVlayerClient, VCallResult } from "@vlayer/sdk";

import { type Address, type Abi } from "viem";
interface UseProverParams {
  addr: Address;
  abi: Abi;
  func: string;
  chainId: number;
}
interface UseProverReturn {
  prove: (proverArgs: string[]) => Promise<string | undefined>;
  provingError: string | null;
  proof: VCallResult | null;
}

const useProver = ({
  addr,
  abi,
  func,
  chainId,
}: UseProverParams): UseProverReturn => {
  const [provingHash, setProvingHash] = useState<string | null>(null);
  const [provingError, setProvingError] = useState<string | null>(null);
  const [proof, setProof] = useState(null);

  const vlayer = useMemo(
    () =>
      createVlayerClient({
        url: import.meta.env.VITE_PROVER_URL,
      }),
    [],
  );

  const prove = async (args: string[]) => {
    try {
      const { hash } = await vlayer.prove({
        address: addr,
        proverAbi: abi,
        functionName: func,
        args,
        chainId,
      });

      setProvingHash(hash);

      return hash;
    } catch (err) {
      setProvingError("Cannot start proving, check logs");
      console.error(err);
    }
  };

  const waitForProof = async (hash: string) => {
    try {
      console.log("Waiting for proving result: ", hash);
      const result = await vlayer.waitForProvingResult({ hash });
      setProof(result);
      console.log("Proof ready:", result);
    } catch (err) {
      setProvingError("Cannot finalize proving, check logs");
      console.error(err);
    }
  };

  useEffect(() => {
    if (provingHash) waitForProof(provingHash);
  }, [provingHash]);

  return {
    prove,
    provingError,
    proof,
  };
};

export default useProver;
