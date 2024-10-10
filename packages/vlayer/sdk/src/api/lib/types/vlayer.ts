import {
  Abi,
  AbiFunction,
  AbiStateMutability,
  Address,
  ContractFunctionArgs,
  ContractFunctionName,
  Hex,
} from "viem";

import { type ProverCallCommitment } from "types/webProofProvider.ts";

type Calldata = string;

export type CallParams = {
  to: Address;
  data: Calldata;
};

export type CallContext = {
  chain_id: number; // 31337
};

export interface Proof {
  length: bigint;
  seal: {
    verifierSelector: Hex;
    seal: [Hex, Hex, Hex, Hex, Hex, Hex, Hex, Hex];
    mode: number;
  };
  dynamicParamsOffsets: [
    bigint,
    bigint,
    bigint,
    bigint,
    bigint,
    bigint,
    bigint,
    bigint,
    bigint,
    bigint,
  ];
  assumptions: {
    proverContractAddress: Address;
    functionSelector: Hex;
    settleBlockHash: Hex;
    settleBlockNumber: bigint;
  };
}

export interface VCallResult {
  evm_call_result: Hex;
  proof: Proof;
}

export interface VCallResponse {
  jsonrpc: string;
  result: VCallResult;
  id: number;
}

// Add more methods here
export type VlayerClient = {
  prove: <
    T extends readonly [AbiFunction, ...Abi[number][]],
    F extends ContractFunctionName<T>,
  >(
    args: VlayerClientProveArgs<T, F>,
  ) => { hash: string };
  waitForProvingResult: ({
    hash,
  }: {
    hash: string;
  }) => Promise<{ proof: Proof; result: unknown[] }>;
};

export type VlayerClientProveArgs<
  T extends readonly [AbiFunction, ...Abi[number][]],
  F extends ContractFunctionName<T>,
> = ProverCallCommitment<T, F> & {
  args: ContractFunctionArgs<T, AbiStateMutability, F>;
};
