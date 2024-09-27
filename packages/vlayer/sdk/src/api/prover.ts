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

import { type CallContext, type CallParams, Proof } from "types/vlayer";
import { v_call } from "./v_call";
import { testChainId1 } from "./helpers";
import { ContractSpec } from "types/ethereum";

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
  chainId = testChainId1,
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
  } = await v_call(call, context);

  const returnValue = decodeFunctionResult({
    abi: abi as Abi,
    data: evm_call_result,
    functionName: functionName as string,
  });

  addDynamicParamsOffsets(abi, functionName, proof);

  return { proof, returnValue: returnValue as `0x${string}`[] };
}

function addDynamicParamsOffsets(
  abi: Abi,
  functionName: string | undefined,
  proof: Proof,
) {
  const proverFunction = abi.find(
    (f) => f.type === "function" && f.name === functionName,
  ) as AbiFunction;

  if (proverFunction?.outputs && proverFunction.outputs.length > 0) {
    const secondVerifyMethodParamType = proverFunction.outputs[0].type;

    if (secondVerifyMethodParamType === "string") {
      proof.dynamicParamsOffsets[0] = BigInt(32);
    }
  }
}
