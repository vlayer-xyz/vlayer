import {
  type Abi,
  AbiStateMutability,
  type Address,
  ContractFunctionArgs,
  ContractFunctionName,
  decodeFunctionResult,
  DecodeFunctionResultReturnType,
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

export type ProveResult<T> =
  | {
      ok: true;
      proof: Proof;
      returnValue: T;
    }
  | {
      ok: false;
      error: unknown;
      proof: undefined;
      returnValue: undefined;
    };

export async function prove<T extends Abi, F extends ContractFunctionName<T>>(
  prover: Address,
  abi: T,
  functionName: F,
  args: ContractFunctionArgs<T, AbiStateMutability, F>,
  chainId = testChainId1,
): Promise<
  ProveResult<
    DecodeFunctionResultReturnType<
      T,
      F,
      ContractFunctionArgs<T, AbiStateMutability, F>
    >
  >
> {
  const calldata = encodeFunctionData({
    abi,
    functionName,
    args,
  });

  const call: CallParams = { to: prover, data: calldata };
  const context: CallContext = {
    chain_id: chainId,
  };

  const vCallResponse = await v_call(call, context);

  if ("error" in vCallResponse) {
    return {
      ok: false,
      error: vCallResponse.error,
      proof: undefined,
      returnValue: undefined,
    };
  }

  const returnValue = decodeFunctionResult({
    abi,
    functionName,
    data: vCallResponse.result.evm_call_result,
  });

  addDynamicParamsOffsets(abi, functionName, vCallResponse.result.proof);

  return { ok: true, proof: vCallResponse.result.proof, returnValue };
}

function addDynamicParamsOffsets(abi: Abi, functionName: string, proof: Proof) {
  const proverFunction = abi.filter(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (it: any) => it.type === "function" && it.name === functionName,
  );

  if (
    proverFunction.length > 0 &&
    proverFunction[0].outputs &&
    proverFunction[0].outputs.length > 0
  ) {
    const secondVerifyMethodParamType = proverFunction[0].outputs[0].type;

    if (secondVerifyMethodParamType === "string") {
      proof.dynamicParamsOffsets[0] = BigInt(32);
    }
  }
}
