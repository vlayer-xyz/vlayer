import { useCallProver, useWaitForProvingResult } from "@vlayer/react";
import proverSpec from "../../../../out/AverageBalance.sol/AverageBalance";
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
    functionName: "averageBalanceOf",
    chainId: 31337,
  });

  const { data: result, error: provingResultError } =
    useWaitForProvingResult(provingHash);

  useEffect(() => {
    if (result && Array.isArray(result)) {
      setProverResult(JSON.stringify(result));
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
