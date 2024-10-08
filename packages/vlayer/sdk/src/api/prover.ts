import {
  type Abi,
  AbiFunction,
  AbiStateMutability,
  type Address,
  ContractFunctionArgs,
  ContractFunctionName,
  decodeFunctionResult,
  encodeFunctionData,
} from "viem";

import { type CallContext, type CallParams } from "types/vlayer";
import { v_call } from "./v_call";
import { ContractSpec } from "types/ethereum";
import { foundry } from "viem/chains";

export async function getContractSpec(file: string): Promise<ContractSpec> {
  return Bun.file(file).json();
}

export async function prove<
  T extends readonly [AbiFunction, ...Abi[number][]],
  F extends ContractFunctionName<T>,
>(
  prover: Address,
  abi: T,
  functionName: F,
  args: ContractFunctionArgs<T, AbiStateMutability, F>,
  chainId: number = foundry.id,
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
  };

  const {
    result: { proof, evm_call_result },
  } = await v_call(call, context, url);

  const [, ...returnValue] = decodeFunctionResult({
    abi: abi as Abi,
    data: evm_call_result,
    functionName: functionName as string,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  }) as any[];

  return { proof, returnValue };
}
