import {
  type Abi,
  AbiStateMutability,
  type Address,
  ContractFunctionArgs,
  ContractFunctionName,
  decodeFunctionResult,
  encodeFunctionData,
  type Hex,
} from "viem";

import {type CallContext, type CallParams, v_call} from "./v_call";
import {testChainId1} from "./helpers";

type Bytecode = {
  object: Hex,
}

export type ContractSpec = {
  abi: Abi,
  bytecode: Bytecode,
}

export type ContractArg = number | string | boolean;

export async function getContractSpec(file: string): Promise<ContractSpec> {
  return Bun.file(file).json();
}

export async function prove<T extends Abi, F extends ContractFunctionName<T>>(prover: Address, abi: T, functionName: F, args: ContractFunctionArgs<T, AbiStateMutability, F>, chainId = testChainId1) {
  const calldata = encodeFunctionData({
    abi,
    functionName,
    args
  });

  const call: CallParams = {to: prover, data: calldata};
  const context: CallContext = {
    chain_id: chainId,
  };

  const response = (await v_call(call, context)).result;
  const proof = response.proof;
  const returnValue = decodeFunctionResult({
    abi,
    functionName,
    data: response.evm_call_result,
  })

  return {proof, returnValue};
}
