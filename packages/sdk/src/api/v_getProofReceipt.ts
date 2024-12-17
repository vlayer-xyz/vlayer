import {
  type VGetProofReceiptParams,
  type VGetProofReceiptResponse,
} from "types/vlayer";
import { parseVCallResponseError } from "./lib/errors";

function v_getProofReceiptBody(params: VGetProofReceiptParams) {
  return {
    method: "v_getProofReceipt",
    params: params,
    id: 1,
    jsonrpc: "2.0",
  };
}

const log = (...args: unknown[]) => {
  console.log("v_getProofReceipt: ", ...args);
};

export async function v_getProofReceipt(
  params: VGetProofReceiptParams,
  url: string = "https://test-prover.vlayer.xyz",
): Promise<VGetProofReceiptResponse> {
  const response = await fetch(url, {
    method: "POST",
    body: JSON.stringify(v_getProofReceiptBody(params)),
    headers: { "Content-Type": "application/json" },
  });
  log("response", response);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const response_json = await response.json();
  log("response_json", response_json);
  assertObject(response_json);
  if ("error" in response_json) {
    throw parseVCallResponseError(
      response_json.error as { message: string | undefined },
    );
  }
  return response_json as Promise<VGetProofReceiptResponse>;
}

function assertObject(x: unknown): asserts x is object {
  if (typeof x !== "object") {
    throw new Error("Expected object");
  }
}
