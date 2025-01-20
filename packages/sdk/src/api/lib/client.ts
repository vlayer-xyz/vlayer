import {
  type VCallResponse,
  type VlayerClient,
  type BrandedHash,
} from "types/vlayer";
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
  type Hex,
} from "viem";
import {
  ExtensionMessageType,
  ZkProvingStatus,
  type PresentationJSON,
} from "../../web-proof-commons";
import { type ContractFunctionArgsWithout } from "types/viem";
import { type ProveArgs } from "types/vlayer";

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
      token,
      args,
    }: ProveArgs<T, F>) => {
      webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Proving);

      const response = prove(
        address,
        proverAbi,
        functionName,
        args,
        chainId,
        gasLimit,
        url,
        token,
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
      try {
        const [proof_data] = await waitForProof(
          hash,
          url,
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
            data: proof_data.evm_call_result,
            functionName,
          }),
        );

        webProofProvider.notifyZkProvingStatus(ZkProvingStatus.Done);

        return [proof_data.proof, ...result] as ContractFunctionReturnType<
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
      token,
      args,
    }) {
      const webProofPlaceholder = args[0];
      const commitmentArgs = args.slice(1) as ContractFunctionArgsWithout<
        typeof proverAbi,
        typeof functionName,
        { name: "webProof" }
      >;

      const webProofPromise: Promise<PresentationJSON> = new Promise(
        (resolve, reject) => {
          webProofProvider.addEventListeners(
            ExtensionMessageType.ProofDone,
            ({ payload: { proof } }) => {
              resolve(proof);
            },
          );

          webProofProvider.addEventListeners(
            ExtensionMessageType.ProofError,
            ({ payload: { error } }) => {
              reject(new Error(error));
            },
          );
        },
      );

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
        token,
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
