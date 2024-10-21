import {
  Abi,
  AbiFunction,
  AbiStateMutability,
  Address,
  ContractFunctionArgs,
  ContractFunctionName,
  Hex,
} from "viem";

type Calldata = string;

export type CallParams = {
  to: Address;
  data: Calldata;
};

export type CallContext = {
  chain_id: number; // 31337
};

export type Proof = {
  length: bigint;
  seal: {
    verifierSelector: Hex;
    seal: readonly [Hex, Hex, Hex, Hex, Hex, Hex, Hex, Hex];
    mode: number;
  };
  callAssumptions: {
    proverContractAddress: Address;
    functionSelector: Hex;
    settleBlockHash: Hex;
    settleBlockNumber: bigint;
  };
};

export interface VCallResult {
  evm_call_result: Hex;
  proof: Proof;
}

export interface VCallResponse {
  jsonrpc: string;
  result: VCallResult;
  id: number;
}

export type VlayerClient = {
  prove: <
    T extends readonly [AbiFunction, ...Abi[number][]],
    F extends ContractFunctionName<T>,
  >(args: {
    address: Hex;
    proverAbi: T;
    functionName: F;
    chainId: number;
    args: ContractFunctionArgs<T, AbiStateMutability, F>;
  }) => Promise<{ hash: string }>;
  waitForProvingResult: ({
    hash,
  }: {
    hash: string;
  }) => Promise<[Proof, ...unknown[]]>;
};
