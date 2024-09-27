import { Address, Hex } from "viem";

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
  commitment: {
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
  prove: () => void;
};
