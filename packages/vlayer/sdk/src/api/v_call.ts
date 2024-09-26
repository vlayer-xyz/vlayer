import { CallContext, CallParams, VCallResponse } from "types/vlayer";

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

  const response_json = await response.json();

  //TODO we should launch some schema validation here
  assertObject(response_json);

  if ("error" in response_json) {
    throw new Error(
      `Error response: ${(response_json.error as { message: string }).message || "unknown error"}`,
    );
  }

  return response_json as Promise<VCallResponse>;
}

function assertObject(x: unknown): asserts x is object {
  if (typeof x !== "object") {
    throw new Error("Expected object");
  }
}
