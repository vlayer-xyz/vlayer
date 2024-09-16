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

import { type CallContext, type CallParams, Proof, v_call } from "./v_call";
import { testChainId1 } from "./helpers";

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
  | Address
  | number[]
  | string[]
  | boolean[]
  | bigint[]
  | Address[];

export async function getContractSpec(file: string): Promise<ContractSpec> {
  return Bun.file(file).json();
}

export async function prove<T extends Abi, F extends ContractFunctionName<T>>(
  prover: Address,
  abi: T,
  functionName: F,
  args: ContractFunctionArgs<T, AbiStateMutability, F>,
  chainId = testChainId1,
) {
  const calldata = encodeFunctionData({
    abi,
    functionName,
    args,
  });

  const call: CallParams = { to: prover, data: calldata };
  const context: CallContext = {
    chain_id: chainId,
  };

  const {
    result: { proof, evm_call_result },
  } = await v_call(call, context);

  const returnValue = decodeFunctionResult({
    abi,
    functionName,
    data: evm_call_result,
  });

  addDynamicParamsOffsets(proof, abi);

  return { proof, returnValue };
}

function addDynamicParamsOffsets(proof: Proof, abi: Abi) {
  const mainFunction = abi.filter(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (it: any) => it.type === "function" && it.name === "main",
  );

  if (
    mainFunction.length > 0 &&
    mainFunction[0].outputs &&
    mainFunction[0].outputs.length > 0
  ) {
    const secondVerifyMethodParamType = mainFunction[0].outputs[0].type;

    if (secondVerifyMethodParamType === "string") {
      proof.dynamicParamsOffsets[0] = BigInt(32);
    }
  }
}
