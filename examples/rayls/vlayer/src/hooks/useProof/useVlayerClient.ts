import { createVlayerClient } from "@vlayer/sdk";
import { useMemo } from "react";
import { Abi } from "viem";

export const useVlayerClient = (proverAbi: Abi, chainId: number) => {
  const vlayer = useMemo(() => {
    const client = createVlayerClient({
      url: import.meta.env.VITE_PROVER_URL,
    });
    return {
      zkProve: async (args: unknown[]) => {
        const hash = await client.prove({
          address: import.meta.env.VITE_PROVER_ADDRESS,
          functionName: "main",
          proverAbi,
          args: args,
          chainId: chainId,
        });
        return await client.waitForProvingResult(hash);
      },
    };
  }, [proverAbi, chainId]);
  return vlayer;
};
