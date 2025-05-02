import {
  handleProverResponseError,
  handleAuthErrors,
} from "./lib/handleErrors";
import { InvalidProverResponseError } from "./lib/errors";
import { validateJrpcResponse } from "./lib/jrpc";
import { versionsSchema, type Versions } from "types/vlayer";
import debug from "debug";

const log = debug("vlayer:v_versions");

const v_versionsBody = {
  method: "v_versions",
  params: [],
  id: 1,
  jsonrpc: "2.0",
};

export async function v_versions(
  url: string = "http://127.0.0.1:3000",
  token?: string,
): Promise<Versions> {
  console.log("v_versions called with url:", url, "and token:", token);

  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  if (token !== undefined) {
    headers["Authorization"] = "Bearer " + token;
  }
  console.log("Request headers:", headers);

  const rawResponse = await fetch(url, {
    method: "POST",
    body: JSON.stringify(v_versionsBody),
    headers,
  });
  console.log("Raw response received:", rawResponse);

  const responseJson = await rawResponse.json();
  console.log("Response JSON body:", responseJson);

  if (!rawResponse.ok) {
    console.log("Response not OK, status:", rawResponse.status);
    throw handleAuthErrors(rawResponse.status, responseJson);
  }

  const response = validateJrpcResponse(responseJson);
  console.log("Validated JRPC response:", response);

  if (response.error !== undefined) {
    console.log("Prover response error:", response.error);
    throw handleProverResponseError(response.error);
  }

  const result = versionsSchema.safeParse(response.result);
  console.log("Schema validation result:", result);

  if (!result.success) {
    console.log("Invalid prover response:", response.result);
    throw new InvalidProverResponseError("v_versions", response.result);
  }

  console.log("Returning parsed result:", result.data);
  return result.data;
}
