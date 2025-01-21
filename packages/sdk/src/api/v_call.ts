import {
  type CallContext,
  type CallParams,
  type VCallResponse,
} from "types/vlayer";
import { parseVCallResponseError } from "./lib/errors";
import Debug from "debug";

const log = Debug("vlayer:v_call");

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
  url: string = "http://127.0.0.1:3000",
): Promise<VCallResponse> {
  const response = await fetch(url, {
    method: "POST",
    body: JSON.stringify(v_callBody(call, context)),
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
  return response_json as Promise<VCallResponse>;
}

function assertObject(x: unknown): asserts x is object {
  if (typeof x !== "object") {
    throw new Error("Expected object");
  }
}
