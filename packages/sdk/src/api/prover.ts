import {
  type Abi,
  type AbiStateMutability,
  type Address,
  type ContractFunctionArgs,
  type ContractFunctionName,
  encodeFunctionData,
  type Hex,
} from "viem";
import {
  type CallContext,
  type CallParams,
  type BrandedHash,
  type VGetProofReceiptParams,
} from "types/vlayer";
import { v_call } from "./v_call";
import { v_getProofReceipt } from "./v_getProofReceipt";
import { foundry } from "viem/chains";
import { v_versions } from "./v_versions";
import { checkVersionCompatibility } from "./utils/versions";
import meta from "../../package.json" with { type: "json" };
const sdkVersion = meta.version;

export interface ProveOptions {
  preverifyVersions?: boolean;
}

async function preverifyVersions(url: string, shouldPreverify: boolean) {
  if (shouldPreverify) {
    const proverVersions = await v_versions(url);
    checkVersionCompatibility(proverVersions.result.api_version, sdkVersion);
  }
}

export async function prove<T extends Abi, F extends ContractFunctionName<T>>(
  prover: Address,
  abi: T,
  functionName: F,
  args: ContractFunctionArgs<T, AbiStateMutability, F>,
  chainId: number = foundry.id,
  gasLimit: number = 1_000_000,
  url: string = "http://127.0.0.1:3000",
  userToken?: string,
  options: ProveOptions = { preverifyVersions: false },
) {
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
  const fullUrl = url.concat(
    userToken !== undefined ? "/?token=" + userToken : "",
  );
  return v_call(call, context, fullUrl);
}

export async function getProofReceipt<
  T extends Abi,
  F extends ContractFunctionName<T>,
>(hash: BrandedHash<T, F>, url: string = "http://127.0.0.1:3000") {
  const params: VGetProofReceiptParams = {
    hash: hash.hash as Hex,
  };
  return v_getProofReceipt(params, url);
}
