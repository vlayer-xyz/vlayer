import { VCallResponse, VlayerClient, BrandedHash } from "types/vlayer";
import { WebProofProvider } from "types/webProofProvider";

import { prove } from "../prover";
import { createExtensionWebProofProvider } from "../webProof";
import {
  type Abi,
  AbiStateMutability,
  ContractFunctionArgs,
  ContractFunctionName,
  ContractFunctionReturnType,
  decodeFunctionResult,
  Hex,
} from "viem";
import { ZkProvingStatus } from "../../web-proof-commons";
import { ContractFunctionArgsWithout } from "types/viem";

function dropEmptyProofFromArgs(args: unknown) {
  if (Array.isArray(args)) {
    return args.slice(1) as unknown[];
  }
  return [];
}

async function getHash(
  vcall_response: Promise<VCallResponse>,
): Promise<[Hex, VCallResponse]> {
  const result = await vcall_response;
  return [result.result.hash, result];
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
    prove: async ({
      address,
      functionName,
      chainId,
      gasLimit,
      proverAbi,
      args,
    }) => {
      webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Proving);
      const response = prove(
        address,
        proverAbi,
        functionName,
        args,
        chainId,
        gasLimit,
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
      const [hash, result_promise] = await getHash(response);
      resultHashMap.set(hash, [
        Promise.resolve(result_promise),
        proverAbi,
        functionName,
      ]);
      return { hash } as BrandedHash<typeof proverAbi, typeof functionName>;
    },

    waitForProvingResult: async <
      T extends Abi,
      F extends ContractFunctionName<T>,
    >({
      hash,
    }: BrandedHash<T, F>): Promise<
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
          functionName,
        }),
      );

      return [proof, ...result] as ContractFunctionReturnType<
        T,
        AbiStateMutability,
        F
      >;
    },

    proveWeb: async function ({
      address,
      proverAbi,
      functionName,
      chainId,
      args,
    }) {
      const webProofPlaceholder = args[0];
      const commitmentArgs = args.slice(1) as ContractFunctionArgsWithout<
        typeof proverAbi,
        typeof functionName,
        { name: "webProof" }
      >;

      const webProof = await webProofProvider.getWebProof({
        proverCallCommitment: {
          address,
          proverAbi,
          functionName,
          commitmentArgs,
          chainId,
        },
        logoUrl: webProofPlaceholder.logoUrl,
        steps: webProofPlaceholder.steps,
      });

      const hash = await this.prove({
        address,
        functionName,
        chainId,
        proverAbi,
        args: [
          {
            webProofJson: JSON.stringify({
              tls_proof: webProof,
              notary_pub_key: webProofPlaceholder.notaryPubKey,
            }),
          },
          ...commitmentArgs,
        ] as ContractFunctionArgs<
          typeof proverAbi,
          AbiStateMutability,
          typeof functionName
        >,
      });
      return hash;
    },
  };
};
