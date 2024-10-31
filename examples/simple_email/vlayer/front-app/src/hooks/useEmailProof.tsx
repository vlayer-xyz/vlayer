import { useMemo, useEffect, useState } from "react";
import { createVlayerClient, preverifyEmail } from "@vlayer/sdk";
import { getStrFromFile } from "../lib/utils";

import type { Address } from "viem";

const useEmailProof = ({ proverAddr, proverAbi, proverFunc, chainId }) => {
  const [isReady, setIsReady] = useState(false);
  const [provingHash, setProvingHash] = useState("");
  const [provingError, setProvingError] = useState(null);
  const [proof, setProof] = useState(null);

  const vlayer = useMemo(
    () =>
      createVlayerClient({
        url: import.meta.env.VITE_PROVER_URL,
      }),
    [],
  );

  const prove = async (emlFile, proverArgs) => {
    try {
      setIsReady(false);
      const eml = await getStrFromFile(emlFile);
      const email = await preverifyEmail(eml);

      const { hash } = await vlayer.prove({
        address: proverAddr,
        proverAbi: proverAbi,
        functionName: proverFunc,
        args: [email, ...proverArgs],
        chainId,
      });

      setProvingHash(hash);

      return hash;
    } catch (err) {
      setProvingError("Cannot start proving, check logs");
      console.error(err);
    }
  };

  const waitForProof = async (hash) => {
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

export default useEmailProof;
