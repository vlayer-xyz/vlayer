import { VCallResponse, VlayerClient } from "types/vlayer";
import { WebProofProvider } from "types/webProofProvider";

import { prove } from "../prover";
import { createExtensionWebProofProvider } from "../webProof";
import {
  type Abi,
  decodeFunctionResult,
  ContractFunctionReturnType,
  AbiStateMutability,
  ContractFunctionName,
} from "viem";
import { Branded } from "../../web-proof-commons/utils.ts";

function dropEmptyProofFromArgs(args: unknown) {
  if (Array.isArray(args)) {
    return args.slice(1) as unknown[];
  }
  return [];
}

function generateRandomHash() {
  let hash = "0x";
  for (let i = 0; i < 40; ++i) {
    hash += Math.floor(Math.random() * 16).toString(16);
  }
  return hash;
}

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
  console.log("createVlayerClient with", url, webProofProvider);
  const resultHashMap = new Map<
    string,
    [Promise<VCallResponse>, Abi, string]
  >();

  return {
    // eslint-disable-next-line @typescript-eslint/require-await
    prove: async ({ address, functionName, chainId, proverAbi, args }) => {
      const result_promise = prove(
        address,
        proverAbi,
        functionName,
        args,
        chainId,
        url,
      );
      const hash = generateRandomHash();
      resultHashMap.set(hash, [result_promise, proverAbi, functionName]);
      return { hash };
    },

    waitForProvingResult: async <
      T extends Abi,
      F extends ContractFunctionName<T>,
    >({
      hash,
    }: Branded<{ hash: string }, [T, F]>): Promise<
      ContractFunctionReturnType<T, AbiStateMutability, F>
    > => {
      const savedProvingData = resultHashMap.get(hash);
      if (!savedProvingData) {
        throw new Error("No result found for hash " + hash);
      }

      const [result_promise, proverAbi, functionName] = savedProvingData;
      const {
        result: { proof, evm_call_result },
      } = await result_promise;

      const result = dropEmptyProofFromArgs(
        decodeFunctionResult({
          abi: proverAbi,
          data: evm_call_result,
          functionName: functionName,
        }),
      );

      return [proof, ...result] as ContractFunctionReturnType<
        T,
        AbiStateMutability,
        F
      >;
    },
  };
};
