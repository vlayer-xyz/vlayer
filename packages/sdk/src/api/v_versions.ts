import { Client } from "./utils/JRpcClient";

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
  const client = new Client(url, token);
  const response = await client.send("v_versions", {});
  assertResponseObject(response);
  return response;
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
