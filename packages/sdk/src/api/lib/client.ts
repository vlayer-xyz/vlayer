import { type VlayerClient, type BrandedHash } from "types/vlayer";
import { type WebProofProvider } from "types/webProofProvider";
import { prove, waitForProof } from "../prover";
import { createExtensionWebProofProvider } from "../webProof";
import {
  type Abi,
  type AbiStateMutability,
  type ContractFunctionArgs,
  type ContractFunctionName,
  type ContractFunctionReturnType,
  decodeFunctionResult,
} from "viem";
import {
  ExtensionMessageType,
  ZkProvingStatus,
  type PresentationJSON,
} from "@vlayer/web-proof-commons";
import { type ContractFunctionArgsWithout } from "types/viem";
import { type ProveArgs } from "types/vlayer";

function dropEmptyProofFromArgs(args: unknown) {
  if (Array.isArray(args)) {
    return args.slice(1) as unknown[];
  }
  return [];
}

export const createVlayerClient = (
  {
    url = "http://127.0.0.1:3000",
    webProofProvider,
    token,
  }: {
    url?: string;
    webProofProvider?: WebProofProvider;
    token?: string;
  } = {
    url: "http://127.0.0.1:3000",
  },
): VlayerClient => {
  const resultHashMap = new Map<string, [Abi, string]>();

  if (!webProofProvider) {
    webProofProvider = createExtensionWebProofProvider({ jwtToken: token });
  }

  return {
    prove: async <T extends Abi, F extends ContractFunctionName<T>>({
      address,
      proverAbi,
      functionName,
      chainId,
      gasLimit,
      args,
    }: ProveArgs<T, F>) => {
      webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Proving);

      const hash = await prove(
        address,
        proverAbi,
        functionName,
        args,
        chainId,
        url,
        gasLimit,
        token,
      ).catch((e) => {
        webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Error);
        throw e;
      });

      resultHashMap.set(hash.hash, [proverAbi, functionName]);
      return hash;
    },

    waitForProvingResult: async <
      T extends Abi,
      F extends ContractFunctionName<T>,
    >({
      hash,
      numberOfRetries = 900,
      sleepDuration = 1000,
    }: {
      hash: BrandedHash<T, F>;
      numberOfRetries?: number;
      sleepDuration?: number;
    }): Promise<ContractFunctionReturnType<T, AbiStateMutability, F>> => {
      try {
        const { data } = await waitForProof(
          hash,
          url,
          token,
          numberOfRetries,
          sleepDuration,
        );
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

        webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Done);

        return [data.proof, ...result] as ContractFunctionReturnType<
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
      gasLimit,
      args,
    }) {
      const webProofPlaceholder = args[0];
      const commitmentArgs = args.slice(1) as ContractFunctionArgsWithout<
        typeof proverAbi,
        typeof functionName,
        { name: "webProof" }
      >;

      const webProofPromise: Promise<{
        presentationJson: PresentationJSON;
        decodedTranscript: {
          sent: string;
          recv: string;
        };
      }> = new Promise((resolve, reject) => {
        webProofProvider.addEventListeners(
          ExtensionMessageType.ProofDone,
          ({ payload: { presentationJson, decodedTranscript } }) => {
            resolve({ presentationJson, decodedTranscript });
          },
        );

        webProofProvider.addEventListeners(
          ExtensionMessageType.ProofError,
          ({ payload: { error } }) => {
            reject(new Error(error));
          },
        );
      });

      webProofProvider.requestWebProof({
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

      const webProof = await webProofPromise;

      const hash = await this.prove({
        address,
        functionName,
        chainId,
        gasLimit,
        proverAbi,
        args: [
          {
            webProofJson: JSON.stringify({
              presentationJson: webProof.presentationJson,
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
