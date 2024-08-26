import {Hex, type Address} from "viem";

type Calldata = string;

export type CallParams = {
  to: Address,
  data: Calldata
}

export type CallContext = {
  block_no: number, // latest
  chain_id: number // 31337
}

export interface Proof {
  evm_call_result: Hex,
  function_selector: string,
  prover_contract_address: Address,
  seal: Hex,
}

export interface VCallResponse {
  jsonrpc: string;
  result: {
    result: Proof
  };
  id: number;
}


function v_callBody(call: CallParams, context: CallContext) {
  return {
    method: "v_call",
    params: [call, context],
    id: 1,
    jsonrpc: "2.0"
  };
}

export async function v_call(call: CallParams, context: CallContext): Promise<VCallResponse> {
  const response = await fetch("http://127.0.0.1:3000", {
    method: "POST",
    body: JSON.stringify(v_callBody(call, context)),
    headers: {"Content-Type": "application/json"}
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  return await response.json() as any;
}
