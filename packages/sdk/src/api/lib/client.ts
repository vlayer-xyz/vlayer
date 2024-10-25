import { VlayerClient } from "types/vlayer";
import { WebProofProvider } from "types/webProofProvider";

import { prove } from "../prover";
import { createExtensionWebProofProvider } from "../webProof";
import {
  type Abi,
  decodeFunctionResult,
  ContractFunctionReturnType,
  AbiStateMutability,
} from "viem";

function dropEmptyProofFromArgs(args: unknown) {
  if (Array.isArray(args)) {
    return args.slice(1) as unknown[];
  }
  return [];
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

  return {
    prove: async ({ address, functionName, chainId, proverAbi, args }) => {
      const result_promise = prove(
        address,
        proverAbi,
        functionName,
        args,
        chainId,
        url,
      );

      const {
        result: { proof, evm_call_result },
      } = await result_promise;

      const result = dropEmptyProofFromArgs(
        decodeFunctionResult({
          abi: proverAbi as Abi,
          data: evm_call_result,
          functionName: functionName as string,
        }),
      );

      return [proof, ...result] as ContractFunctionReturnType<
        typeof proverAbi,
        AbiStateMutability,
        typeof functionName
      >;
    },
  };
};
