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
    token: import.meta.env.VITE_VLAYER_API_TOKEN,
  });

  const { data: result, error: provingResultError } =
    useWaitForProvingResult(provingHash);

  useEffect(() => {
    if (result) {
      const resultsData = [
        result[0],
        result[1],
        String(result[2]), // BigInt balance
      ];
      setProverResult(JSON.stringify(resultsData));
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
