import { useCallProver, useWaitForProvingResult } from "@vlayer/react";
import proverSpec from "../../../../out/SimpleTeleportProver.sol/SimpleTeleportProver";
import { useEffect } from "react";
import { useLocalStorage } from "usehooks-ts";

export const useProver = () => {
  const [, setProverResult] = useLocalStorage("proverResult", "");

  const {
    callProver,
    data: provingHash,
    error: provingError,
  } = useCallProver({
    address: import.meta.env.VITE_PROVER_ADDRESS,
    proverAbi: proverSpec.abi,
    functionName: "crossChainBalanceOf",
    gasLimit: Number(import.meta.env.VITE_GAS_LIMIT),
  });

  const { data: result, error: provingResultError } =
    useWaitForProvingResult(provingHash);

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

  useEffect(() => {
    if (provingError || provingResultError) {
      console.log(
        "error: ",
        provingError?.message || provingResultError?.message,
      );
    }
  }, [provingError, provingResultError]);

  return { callProver, provingHash, result };
};
