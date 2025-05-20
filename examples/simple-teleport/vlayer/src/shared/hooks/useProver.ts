import {
  useChain,
  useCallProver,
  useWaitForProvingResult,
} from "@vlayer/react";
import proverSpec from "../../../../out/SimpleTeleportProver.sol/SimpleTeleportProver";
import { useEffect } from "react";
import { useLocalStorage } from "usehooks-ts";
import { UseChainError, CallProverError } from "../errors/appErrors";

export const useProver = () => {
  const [, setProverResult] = useLocalStorage("proverResult", "");

  const { chain, error: chainError } = useChain(
    import.meta.env.VITE_CHAIN_NAME,
  );

  if (chainError) {
    throw new UseChainError(chainError);
  }

  const {
    callProver,
    data: provingHash,
    error: provingError,
  } = useCallProver({
    address: import.meta.env.VITE_PROVER_ADDRESS,
    proverAbi: proverSpec.abi,
    functionName: "crossChainBalanceOf",
    gasLimit: Number(import.meta.env.VITE_GAS_LIMIT),
    chainId: chain?.id,
  });

  if (provingError) {
    throw new CallProverError(provingError.message);
  }

  const { data: result, error: provingResultError } =
    useWaitForProvingResult(provingHash);

  if (provingResultError) {
    throw new CallProverError(provingResultError.message);
  }

  useEffect(() => {
    if (result && Array.isArray(result)) {
      console.log("result", result);
      setProverResult(
        JSON.stringify(result, (key, value) => {
          if (typeof value === "bigint") {
            return String(value);
          }
          return value as string;
        }),
      );
    }
  }, [result]);

  return { callProver, provingHash, result };
};
