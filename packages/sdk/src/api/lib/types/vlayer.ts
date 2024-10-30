import {
  Abi,
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

export const EMPTY_PROOF: Proof = {
  length: BigInt(0),
  seal: {
    verifierSelector: "0x",
    seal: ["0x", "0x", "0x", "0x", "0x", "0x", "0x", "0x"],
    mode: 0,
  },
  callAssumptions: {
    proverContractAddress: "0x",
    functionSelector: "0x",
    settleBlockHash: "0x",
    settleBlockNumber: BigInt(0),
  },
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

  prove: <T extends Abi, F extends ContractFunctionName<T>>(args: {
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

  proveWeb: <T extends Abi, F extends ContractFunctionName<T>>(args: {
    address: Hex;
    proverAbi: T;
    functionName: F;
    chainId: number;
    args: ContractFunctionArgs<T, AbiStateMutability, F>;
  }) => Promise<[Proof, ...unknown[]]>;
};
