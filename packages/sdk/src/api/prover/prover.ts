import {
  type Abi,
  type AbiStateMutability,
  type Address,
  type ContractFunctionArgs,
  type ContractFunctionName,
  encodeFunctionData,
} from "viem";
import {
  type CallContext,
  type CallParams,
  type BrandedHash,
  type ProofDataWithMetrics,
  type ProofReceipt,
  ProofState,
} from "types/vlayer";
import { match } from "ts-pattern";
import { v_call } from "./v_call";
import { v_getProofReceipt } from "./v_getProofReceipt";
import { foundry } from "viem/chains";
import { v_versions } from "./v_versions";
import { checkVersionCompatibility } from "../utils/versions";
import meta from "../../../package.json" with { type: "json" };
const sdkVersion = meta.version;

export interface ProveOptions {
  preverifyVersions?: boolean;
}

async function preverifyVersions(
  url: string,
  shouldPreverify: boolean,
  token?: string,
) {
  if (shouldPreverify) {
    const proverVersions = await v_versions(url, token);
    checkVersionCompatibility(proverVersions.api_version, sdkVersion);
  }
}

export async function prove<T extends Abi, F extends ContractFunctionName<T>>(
  prover: Address,
  abi: T,
  functionName: F,
  args: ContractFunctionArgs<T, AbiStateMutability, F>,
  chainId: number = foundry.id,
  url: string = "http://127.0.0.1:3000",
  gasLimit: number = 10_000_000,
  token?: string,
  options: ProveOptions = { preverifyVersions: false },
): Promise<BrandedHash<T, F>> {
  await preverifyVersions(url, !!options.preverifyVersions);
  const calldata = encodeFunctionData({
    abi: abi as Abi,
    functionName: functionName as string,
    args: args as readonly unknown[],
  });
  const call: CallParams = { to: prover, data: calldata, gas_limit: gasLimit };
  const context: CallContext = {
    chain_id: chainId,
  };
  const hash = await v_call(call, context, url, token);
  return { hash } as BrandedHash<T, F>;
}

export async function getProofReceipt<
  T extends Abi,
  F extends ContractFunctionName<T>,
>(
  hash: BrandedHash<T, F>,
  url: string = "http://127.0.0.1:3000",
  token?: string,
): Promise<ProofReceipt> {
  const resp = await v_getProofReceipt(hash.hash, url, token);
  handleErrors(resp);
  return resp;
}

const handleErrors = ({ status, state, error }: ProofReceipt) => {
  if (status === 0) {
    match(state)
      .with(ProofState.AllocateGas, () => {
        throw new Error(`Allocating gas failed with error: ${error}`);
      })
      .with(ProofState.Preflight, () => {
        throw new Error(`Preflight failed with error: ${error}`);
      })
      .with(ProofState.Proving, () => {
        throw new Error(`Proving failed with error: ${error}`);
      })
      .exhaustive();
  }
};

export async function waitForProof<
  T extends Abi,
  F extends ContractFunctionName<T>,
>(
  hash: BrandedHash<T, F>,
  url: string,
  token?: string,
  numberOfRetries: number = 900,
  sleepDuration: number = 3000,
): Promise<ProofDataWithMetrics> {
  for (let retry = 0; retry < numberOfRetries; retry++) {
    const { state, data, metrics } = await getProofReceipt(hash, url, token);
    if (state === ProofState.Done) {
      return { data, metrics };
    }
    await sleep(sleepDuration);
  }
  throw new Error(
    `Timed out waiting for ZK proof generation after ${numberOfRetries * sleepDuration}ms. Consider increasing numberOfRetries.`,
  );
}

async function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
