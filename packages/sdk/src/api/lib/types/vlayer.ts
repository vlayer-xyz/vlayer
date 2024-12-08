import { Branded } from "../../../web-proof-commons/utils";
import {
  Abi,
  AbiStateMutability,
  Address,
  ContractFunctionArgs,
  ContractFunctionName,
  ContractFunctionReturnType,
  Hex,
} from "viem";
import { WebProofRequest } from "./webProofProvider";
import { ContractFunctionArgsWithout } from "./viem";

type Calldata = string;

export type CallParams = {
  to: Address;
  data: Calldata;
};

export type CallContext = {
  chain_id: number;
  gas_limit: number;
};

export type BrandedHash<T, F> = Branded<{ hash: string }, [T, F]>;

export type Proof = {
  seal: {
    verifierSelector: Hex;
    seal: readonly [Hex, Hex, Hex, Hex, Hex, Hex, Hex, Hex];
    mode: number;
  };
  callGuestId: Hex;
  length: bigint;
  callAssumptions: {
    proverContractAddress: Address;
    functionSelector: Hex;
    settleBlockHash: Hex;
    settleBlockNumber: bigint;
  };
};

export type VCallResult = Hex;

export interface VCallResponse {
  jsonrpc: string;
  result: VCallResult;
  id: number;
}

export type VGetProofReceiptParams = {
  hash: Hex;
};

export enum VGetProofReceiptStatus {
  pending = "pending",
  done = "done",
}

export interface VGetProofReceiptResult {
  status: VGetProofReceiptStatus;
  data: {
    evm_call_result: Hex;
    proof: Proof;
  };
}

export interface VGetProofReceiptResponse {
  jsonrpc: string;
  result: VGetProofReceiptResult;
  id: number;
}

export type VlayerClient = {
  prove: <T extends Abi, F extends ContractFunctionName<T>>(args: {
    address: Hex;
    proverAbi: T;
    functionName: F;
    chainId?: number;
    gasLimit?: number;
    args: ContractFunctionArgs<T, AbiStateMutability, F>;
  }) => Promise<BrandedHash<T, F>>;

  waitForProvingResult: <
    T extends Abi,
    F extends ContractFunctionName<T>,
  >(args: {
    hash: BrandedHash<T, F>;
    number_of_retries?: number;
    sleep_duration?: number;
  }) => Promise<ContractFunctionReturnType<T, AbiStateMutability, F>>;

  proveWeb: <T extends Abi, F extends ContractFunctionName<T>>(args: {
    address: Hex;
    proverAbi: T;
    functionName: F;
    chainId: number;
    args: [
      WebProofRequest,
      ...ContractFunctionArgsWithout<T, F, { name: "webProof" }>,
    ];
  }) => Promise<BrandedHash<T, F>>;
};
