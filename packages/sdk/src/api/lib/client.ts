import { VCallResponse, VlayerClient } from "types/vlayer";
import { WebProofProvider } from "types/webProofProvider";

import { prove } from "../prover";
import { createExtensionWebProofProvider } from "../webProof";
import { type Abi, decodeFunctionResult } from "viem";
import { ZkProvingStatus } from "../../web-proof-commons";

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

async function getHash() {
  return Promise.resolve(generateRandomHash());
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
  const resultHashMap = new Map<
    string,
    [Promise<VCallResponse>, Abi, string]
  >();

  return {
    prove: async ({ address, functionName, chainId, proverAbi, args }) => {
      webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Proving);
      const result_promise = prove(
        address,
        proverAbi,
        functionName,
        args,
        chainId,
        url,
      )
        .catch((error) => {
          webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Error);
          throw error;
        })
        .then((result) => {
          webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Done);
          return result;
        });
      const hash = await getHash();
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

      const result = dropEmptyProofFromArgs(
        decodeFunctionResult({
          abi: savedProvingData[1],
          data: evm_call_result,
          functionName: savedProvingData[2],
        }),
      );

      return [proof, ...result];
    },
  };
};
