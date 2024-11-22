import {
  type Abi,
  AbiStateMutability,
  type Address,
  ContractFunctionArgs,
  ContractFunctionName,
  encodeFunctionData,
} from "viem";

import { type CallContext, type CallParams } from "types/vlayer";
import { v_call } from "./v_call";
import { foundry } from "viem/chains";

export async function prove<T extends Abi, F extends ContractFunctionName<T>>(
  prover: Address,
  abi: T,
  functionName: F,
  args: ContractFunctionArgs<T, AbiStateMutability, F>,
  chainId: number = foundry.id,
  gasLimit: number = 1_000_000,
  url: string = "http://127.0.0.1:3000",
) {
  const calldata = encodeFunctionData({
    abi: abi as Abi,
    functionName: functionName as string,
    args: args as readonly unknown[],
  });
  const call: CallParams = { to: prover, data: calldata };
  const context: CallContext = {
    chain_id: chainId,
    gas_limit: gasLimit,
  };
  return v_call(call, context, url);
}
