import Debug from "debug";

const log = Debug("vlayer:v_versions");

const v_versionsBody = {
  method: "v_versions",
  params: [],
  id: 1,
  jsonrpc: "2.0",
};

interface VVersionsResponseResult {
  call_guest_id: string;
  chain_guest_id: string;
  api_version: string;
}

export interface VVersionsResponse {
  jsonrpc: string;
  result: VVersionsResponseResult;
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
  log("response", response);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const response_json = await response.json();
  assertResponseObject(response_json);
  return response_json;
}

function isFieldAString(
  x: object,
  field: keyof VVersionsResponseResult,
): boolean {
  return (
    field in x && typeof (x as VVersionsResponseResult)[field] === "string"
  );
}

function assertResponseObject(x: unknown): asserts x is VVersionsResponse {
  if (!x || typeof x !== "object") {
    throw new Error("Expected object");
  }
  if (!("result" in x) || !x.result || typeof x.result !== "object") {
    throw new Error(
      `Unexpected \`v_versions\` response: ${JSON.stringify(x, null, 2)}`,
    );
  }
  if (
    !isFieldAString(x.result, "call_guest_id") ||
    !isFieldAString(x.result, "chain_guest_id") ||
    !isFieldAString(x.result, "api_version")
  ) {
    throw new Error(
      `Unexpected \`v_versions\` response: ${JSON.stringify(x, null, 2)}`,
    );
  }
}
