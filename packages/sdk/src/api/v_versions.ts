import { VCallResponse } from "types/vlayer";

const v_versionsBody = {
  method: "v_versions",
  params: [],
  id: 1,
  jsonrpc: "2.0",
};

export interface VVersionsResponse {
  jsonrpc: string;
  result: {
    call_guest_id: string;
    chain_guest_id: string;
  };
  id: number;
}

export async function v_versions(
  url: string = "http://127.0.0.1:3000",
): Promise<VVersionsResponse> {
  const response = await fetch(url, {
    method: "POST",
    body: JSON.stringify(v_versionsBody),
    headers: { "Content-Type": "application/json" },
  });
  console.log("response", response);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const response_json = await response.json();
  assertResponseObject(response_json);
  return response_json;
}

function assertResponseObject(x: unknown): asserts x is VVersionsResponse {
  if (!x || typeof x !== "object") {
    throw new Error("Expected object");
  }
  if (!("result" in x) || !x.result || typeof x.result !== "object") {
    throw new Error(`Unexpected \`v_versions\` response: ${x}`);
  }
  if (
    !("call_guest_id" in x.result) ||
    typeof x.result.call_guest_id !== "string"
  ) {
    throw new Error(`Unexpected \`v_versions\` response: ${x}`);
  }
}
