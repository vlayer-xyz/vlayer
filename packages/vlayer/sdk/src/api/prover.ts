import {
  type Abi,
  AbiStateMutability,
  type Address,
  ContractFunctionArgs,
  ContractFunctionName,
  decodeAbiParameters,
  decodeFunctionResult,
  encodeFunctionData,
  type Hex,
  parseAbiParameter
} from "viem";

import {type CallContext, type CallParams, v_call, VCallResponse} from "./v_call";
import {testChainId1, client} from "./helpers";

type Bytecode = {
  object: Hex,
}

export type ContractSpec = {
  abi: Abi,
  bytecode: Bytecode,
}

export type ContractArg = number | string | boolean;

const EXECUTION_COMMITMENT_FIELDS_COUNT = 4;
const FIELD_SIZE_IN_BYTES = 32;
const EXECUTION_COMMITMENT_SIZE = EXECUTION_COMMITMENT_FIELDS_COUNT * FIELD_SIZE_IN_BYTES;


export async function getContractSpec(file: string): Promise<ContractSpec> {
  return Bun.file(file).json();
}

export async function prove<T extends Abi, F extends ContractFunctionName<T>>(prover: Address, abi: T, functionName: F, args: ContractFunctionArgs<T, AbiStateMutability, F>, blockNo?: number) {
  const calldata = encodeFunctionData({
    abi,
    functionName,
    args
  });

  const blockNumber = blockNo ?? await client().getBlockNumber();

  const call: CallParams = {to: prover, data: calldata};
  const context: CallContext = {
    block_no: Number(blockNumber),
    chain_id: testChainId1
  };

  const response = await v_call(call, context);
  const proof = await composeProof(response, BigInt(blockNumber));
  const returnValue = decodeFunctionResult({
    abi,
    functionName,
    data: response.result.evm_call_result,
  })

  return {proof, returnValue};
}

async function composeProof(response: VCallResponse, blockNumber: bigint) {
  const length = EXECUTION_COMMITMENT_SIZE + byteLength(response.result.evm_call_result);
  const blockHash = (await client().getBlock({
    blockNumber
  })).hash;

  const {prover_contract_address, seal: encodedSeal, function_selector} = response.result;

  const SEAL_STRUCT = 'struct Seal { bytes32[8] seal; uint8 mode; }';

  const [seal] = decodeAbiParameters([parseAbiParameter([
    'Seal',
    SEAL_STRUCT
  ])], encodedSeal);

  return {
    length: BigInt(length),
    commitment: {
      proverContractAddress: prover_contract_address as Address,
      functionSelector: function_selector as Hex,
      settleBlockNumber: blockNumber,
      settleBlockHash: blockHash
    },
    seal
  }
}

function byteLength(hex: Hex): number {
  return (hex.length - 2) / 2;
}
