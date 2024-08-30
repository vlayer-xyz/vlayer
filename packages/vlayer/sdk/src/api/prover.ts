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

  const {result: {proof, evm_call_result}} = await v_call(call, context);

  const returnValue = decodeFunctionResult({
    abi,
    functionName,
    data: evm_call_result,
  })

  return {proof, returnValue};
}
