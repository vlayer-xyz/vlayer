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
import { WebProofSetup } from "./webProofProvider";
import { ContractFunctionArgsWithout } from "./viem";

type Calldata = string;

export type CallParams = {
  to: Address;
  data: Calldata;
};

export type CallContext = {
  chain_id: number; // 31337
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
    chainId?: number;
    args: ContractFunctionArgs<T, AbiStateMutability, F>;
  }) => Promise<BrandedHash<T, F>>;

  waitForProvingResult: <T extends Abi, F extends ContractFunctionName<T>>(
    hash: BrandedHash<T, F>,
  ) => Promise<ContractFunctionReturnType<T, AbiStateMutability, F>>;

  proveWeb: <T extends Abi, F extends ContractFunctionName<T>>(args: {
    address: Hex;
    proverAbi: T;
    functionName: F;
    chainId: number;
    args: [
      WebProofSetup,
      ...ContractFunctionArgsWithout<T, F, { name: "webProof" }>,
    ];
    notary_pub_key: string;
  }) => Promise<BrandedHash<T, F>>;
};
