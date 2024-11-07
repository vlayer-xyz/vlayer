import { CallContext, CallParams, VCallResponse } from "types/vlayer";
import { VersionError } from "./lib/errors";

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
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  const response_json = await response.json();
  assertObject(response_json);

  if ("error" in response_json) {
    throw parseError(response_json.error as { message: string | undefined });
  }

  return response_json as Promise<VCallResponse>;
}

function parseError({ message }: { message: string | undefined }): Error {
  if (message?.startsWith("Unsupported CallGuestID")) {
    return new VersionError(`${message}
    vlayer uses the daily release cycle, and SDK version must match the proving server version.
    Please run "vlayer update" to update the SDK to the latest version.`);
  }
  return new Error(`Error response: ${message ?? "unknown error"}`);
}

function assertObject(x: unknown): asserts x is object {
  if (typeof x !== "object") {
    throw new Error("Expected object");
  }
}
