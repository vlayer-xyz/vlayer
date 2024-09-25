import {
  type Abi,
  AbiFunction,
  type Address,
  ContractFunctionArgs,
  ContractFunctionName,
  decodeFunctionResult,
  encodeFunctionData,
  type Hex,
} from "viem";

import { type CallContext, type CallParams, Proof, v_call } from "./v_call";

type Bytecode = {
  object: Hex;
};

export type ContractSpec = {
  abi: Abi;
  bytecode: Bytecode;
};

export type ContractArg =
  | number
  | string
  | boolean
  | bigint
  | Address
  | number[]
  | string[]
  | boolean[]
  | bigint[]
  | Address[];

export async function getContractSpec(file: string): Promise<ContractSpec> {
  return Bun.file(file).json();
}

// TODO all those casts here are not acceptable in long term
import { testChainId1 } from "./helpers";

export async function prove<T extends Abi>(
  prover: Address,
  abi: T,
  functionName: ContractFunctionName<T> | undefined,
  args: ContractFunctionArgs<T>,
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
    functionName: functionName,
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
