import { VCallResponse, VlayerClient, BrandedHash } from "types/vlayer";
import { WebProofProvider, WebProofSetup } from "types/webProofProvider";

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

    proveWeb: async function <
      T extends Abi,
      F extends ContractFunctionName<T>,
    >(args: {
      address: Hex;
      proverAbi: T;
      functionName: F;
      chainId: number;
      args: [
        WebProofSetup,
        ...ContractFunctionArgsWithout<T, F, { name: "webProof" }>,
      ];
      notary_pub_key: string;
    }) {
      const webProofPlaceholder = args.args[0];
      const contractArgs = args.args.slice(1) as ContractFunctionArgsWithout<
        T,
        F,
        { name: "webProof" }
      >;

      const webProof = await webProofProvider.getWebProof({
        proverCallCommitment: {
          address: args.address,
          proverAbi: args.proverAbi,
          functionName: args.functionName,
          commitmentArgs: contractArgs,
          chainId: args.chainId,
        },
        logoUrl: webProofPlaceholder.logoUrl,
        steps: webProofPlaceholder.steps,
      });

      const hash = await this.prove({
        address: args.address,
        functionName: args.functionName,
        chainId: args.chainId,
        proverAbi: args.proverAbi,
        args: [
          {
            webProofJson: JSON.stringify({
              tls_proof: webProof,
              notary_pub_key: webProofPlaceholder.notaryPubKey,
            }),
          },
          ...contractArgs,
        ] as ContractFunctionArgs<T, AbiStateMutability, F>,
      });
      return hash;
    },
  };
};
