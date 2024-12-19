import { type Branded } from "../../../web-proof-commons/utils";
import {
  type Abi,
  type AbiStateMutability,
  type Address,
  type ContractFunctionArgs,
  type ContractFunctionName,
  type ContractFunctionReturnType,
  type Hex,
} from "viem";
import { type WebProofRequest } from "./webProofProvider";
import { type ContractFunctionArgsWithout } from "./viem";

type Calldata = string;

export type CallParams = {
  to: Address;
  data: Calldata;
  gas_limit: number;
};

export type CallContext = {
  chain_id: number;
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
  Queued = "queued",
  WaitingForChainProof = "waiting_for_chain_proof",
  Preflight = "preflight",
  Proving = "proving",
  Ready = "ready",
}

export interface VGetProofReceiptResult {
  status: VGetProofReceiptStatus;
  data?: {
    evm_call_result: Hex;
    proof: Proof;
  };
}

export interface VGetProofReceiptResponse {
  jsonrpc: string;
  result: VGetProofReceiptResult;
  id: number;
}

export type ProveArgs<T extends Abi, F extends ContractFunctionName<T>> = {
  address: Hex;
  proverAbi: T;
  functionName: F;
  chainId?: number;
  gasLimit?: number;
  token?: string;
  args: ContractFunctionArgs<T, AbiStateMutability, F>;
};

export type VlayerClient = {
  prove: <T extends Abi, F extends ContractFunctionName<T>>(
    args: ProveArgs<T, F>,
  ) => Promise<BrandedHash<T, F>>;

  waitForProvingResult: <
    T extends Abi,
    F extends ContractFunctionName<T>,
  >(args: {
    hash: BrandedHash<T, F>;
    numberOfRetries?: number;
    sleepDuration?: number;
  }) => Promise<ContractFunctionReturnType<T, AbiStateMutability, F>>;

  proveWeb: <T extends Abi, F extends ContractFunctionName<T>>(args: {
    address: Hex;
    proverAbi: T;
    functionName: F;
    chainId: number;
    token?: string;
    args: [
      WebProofRequest,
      ...ContractFunctionArgsWithout<T, F, { name: "webProof" }>,
    ];
  }) => Promise<BrandedHash<T, F>>;
};
