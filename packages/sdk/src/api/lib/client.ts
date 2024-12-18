import {
  type VCallResponse,
  type VlayerClient,
  type BrandedHash,
  VGetProofReceiptStatus,
} from "types/vlayer";
import { type WebProofProvider } from "types/webProofProvider";

import { prove, getProofReceipt } from "../prover";
import { createExtensionWebProofProvider } from "../webProof";
import {
  type Abi,
  type AbiStateMutability,
  type ContractFunctionArgs,
  type ContractFunctionName,
  type ContractFunctionReturnType,
  decodeFunctionResult,
  type Hex,
} from "viem";
import { ZkProvingStatus } from "../../web-proof-commons";
import { type ContractFunctionArgsWithout } from "types/viem";

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
    prove: async <T extends Abi, F extends ContractFunctionName<T>>({
      address,
      proverAbi,
      functionName,
      chainId,
      gasLimit,
      userToken,
      args,
    }: {
      address: Hex;
      proverAbi: T;
      functionName: F;
      chainId?: number;
      gasLimit?: number;
      userToken?: string;
      args: ContractFunctionArgs<T, AbiStateMutability, F>;
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
        userToken,
      ).catch((e) => {
        webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Error);
        throw e;
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
      numberOfRetries = 240,
      sleepDuration = 1000,
    }: {
      hash: BrandedHash<T, F>;
      numberOfRetries?: number;
      sleepDuration?: number;
    }): Promise<ContractFunctionReturnType<T, AbiStateMutability, F>> => {
      const getProof = async () => {
        for (let retry = 0; retry < numberOfRetries; retry++) {
          const resp = await getProofReceipt(hash, url);
          if (resp.result.status === VGetProofReceiptStatus.ready) {
            if (resp.result.data === undefined) {
              throw new Error(
                "No ZK proof returned from server for hash " + hash.hash,
              );
            }
            return resp.result.data;
          }
          await sleep(sleepDuration);
        }
        throw new Error(
          `Timed out waiting for ZK proof generation after ${numberOfRetries * sleepDuration}ms. Consider increasing numberOfRetries in waitForProvingResult`,
        );
      };
      try {
        const data = await getProof();
        const savedProvingData = resultHashMap.get(hash.hash);
        if (!savedProvingData) {
          throw new Error("No result found for hash " + hash.hash);
        }
        const [proverAbi, functionName] = savedProvingData;

        const result = dropEmptyProofFromArgs(
          decodeFunctionResult({
            abi: proverAbi,
            data: data?.evm_call_result,
            functionName,
          }),
        );

        webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Done);

        return [data?.proof, ...result] as ContractFunctionReturnType<
          T,
          AbiStateMutability,
          F
        >;
      } catch (e) {
        webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Error);
        throw e;
      }
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
            webProofJson: JSON.stringify(webProof),
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
