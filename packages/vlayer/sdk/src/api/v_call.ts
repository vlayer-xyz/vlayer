import { Hex, type Address } from "viem";

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

function v_callBody(call: CallParams, context: CallContext) {
  return {
    method: "v_call",
    params: [call, context],
    id: 1,
    jsonrpc: "2.0",
  };
}

export async function v_call(
  call: CallParams,
  context: CallContext,
): Promise<VCallResponse> {
  const response = await fetch("http://127.0.0.1:3000", {
    method: "POST",
    body: JSON.stringify(v_callBody(call, context)),
    headers: { "Content-Type": "application/json" },
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  return response.json() as Promise<VCallResponse>;
}
