import debug from "debug";

const log = debug("vlayer:v_versions");

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
  token?: string,
): Promise<VVersionsResponse> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  if (token !== undefined) {
    headers["Authorization"] = "Bearer " + token;
  }
  const response = await fetch(url, {
    method: "POST",
    body: JSON.stringify(v_versionsBody),
    headers,
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
