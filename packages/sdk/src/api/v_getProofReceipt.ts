import {
  type VGetProofReceiptParams,
  type VGetProofReceiptResponse,
} from "types/vlayer";
import { parseVCallResponseError } from "./lib/errors";
import { vGetProofReceiptSchema } from "./lib/types/vlayer";
import debug from "debug";

const log = debug("vlayer:v_getProofReceipt");

function v_getProofReceiptBody(params: VGetProofReceiptParams) {
  return {
    method: "v_getProofReceipt",
    params: params,
    id: 1,
    jsonrpc: "2.0",
  };
}

export async function v_getProofReceipt(
  params: VGetProofReceiptParams,
  url: string = "http://127.0.0.1:3000",
  token?: string,
): Promise<VGetProofReceiptResponse> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  if (token !== undefined) {
    headers["Authorization"] = "Bearer " + token;
  }
  const response = await fetch(url, {
    method: "POST",
    body: JSON.stringify(v_getProofReceiptBody(params)),
    headers,
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
  return vGetProofReceiptSchema.parse(response_json);
}

function assertObject(x: unknown): asserts x is object {
  if (typeof x !== "object") {
    throw new Error("Expected object");
  }
}
