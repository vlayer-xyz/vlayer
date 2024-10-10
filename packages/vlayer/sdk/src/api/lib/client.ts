import { VCallResponse, VlayerClient } from "types/vlayer";
import { WebProofProvider } from "types/webProofProvider";

import { prove } from "../prover";
import { createExtensionWebProofProvider } from "../webProof";
import { type Abi, decodeFunctionResult } from "viem";
export const createVlayerClient = (
  {
    url = "http://127.0.0.1:3000",
    webProofProvider = createExtensionWebProofProvider(),
  }: {
    url?: string;
    webProofProvider?: WebProofProvider;
  } = {
    url: "http://127.0.0.1:3000",
    webProofProvider: createExtensionWebProofProvider(),
  },
): VlayerClient => {
  // TODO : implement high level api
  console.log("createVlayerClient with", url, webProofProvider);
  const resultHashMap = new Map<
    string,
    [Promise<VCallResponse>, Abi, string]
  >();

  return {
    prove: ({ address, functionName, chainId, proverAbi, args }) => {
      const result_promise = prove(
        address,
        proverAbi,
        functionName,
        args,
        chainId,
        url,
      );
      const hash = address + functionName + args;
      resultHashMap.set(hash, [result_promise, proverAbi, functionName]);
      return { hash };
    },
    waitForProvingResult: async ({ hash }) => {
      const savedProvingData = resultHashMap.get(hash);
      if (!savedProvingData) {
        throw new Error("No result found for hash " + hash);
      }
      const {
        result: { proof, evm_call_result },
      } = await savedProvingData[0];

      const [, ...result] = decodeFunctionResult({
        abi: savedProvingData[1] as Abi,
        data: evm_call_result,
        functionName: savedProvingData[2] as string,
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      }) as any[];

      return { proof, result };
    },
  };
};
