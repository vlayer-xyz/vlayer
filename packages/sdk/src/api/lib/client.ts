import {
  VCallResponse,
  VlayerClient,
  BrandedHash,
  VGetProofReceiptStatus,
} from "types/vlayer";
import { WebProofProvider } from "types/webProofProvider";

import { prove, getProofReceipt } from "../prover";
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

async function getHash(vcall_response: Promise<VCallResponse>): Promise<Hex> {
  const result = await vcall_response;
  return result.result;
}

async function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
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
  const resultHashMap = new Map<string, [Abi, string]>();

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
      const hash = await getHash(response);
      resultHashMap.set(hash, [proverAbi, functionName]);
      return { hash } as BrandedHash<typeof proverAbi, typeof functionName>;
    },

    waitForProvingResult: async <
      T extends Abi,
      F extends ContractFunctionName<T>,
    >({
      hash,
      numberOfRetries = 120,
      sleepDuration = 1000,
    }: {
      hash: BrandedHash<T, F>;
      numberOfRetries?: number;
      sleepDuration?: number;
    }): Promise<ContractFunctionReturnType<T, AbiStateMutability, F>> => {
      const getProof = async () => {
        for (let retry = 0; retry < numberOfRetries; retry++) {
          const resp = await getProofReceipt(hash, url);
          if (resp.result.status === VGetProofReceiptStatus.done) {
            return resp.result.data;
          }
          await sleep(sleepDuration);
        }
        throw new Error(
          `Timed out waiting for ZK proof generation after {numberOfRetries * sleepDuration}ms. Consider increasing numberOfRetries in waitForProvingResult`,
        );
      };
      const data = await getProof();
      const savedProvingData = resultHashMap.get(hash.hash);
      if (!savedProvingData) {
        throw new Error("No result found for hash " + hash.hash);
      }
      const [proverAbi, functionName] = savedProvingData;

      const result = dropEmptyProofFromArgs(
        decodeFunctionResult({
          abi: proverAbi,
          data: data.evm_call_result,
          functionName,
        }),
      );

      return [data.proof, ...result] as ContractFunctionReturnType<
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
