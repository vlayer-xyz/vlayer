import {
  handleProverResponseError,
  handleAuthErrors,
} from "../lib/handleErrors";
import { InvalidProverResponseError } from "../lib/errors";
import { validateJrpcResponse } from "../lib/jrpc";
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
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  if (token !== undefined) {
    headers["Authorization"] = "Bearer " + token;
  }
  const rawResponse = await fetch(url, {
    method: "POST",
    body: JSON.stringify(v_versionsBody),
    headers,
  });
  log("raw response: ", rawResponse);

  const responseJson = await rawResponse.json();
  log("response body: ", responseJson);

  if (!rawResponse.ok) {
    throw handleAuthErrors(rawResponse.status, responseJson);
  }

  const response = validateJrpcResponse(responseJson);

  if (response.error !== undefined) {
    throw handleProverResponseError(response.error);
  }

  const result = versionsSchema.safeParse(response.result);
  if (!result.success) {
    throw new InvalidProverResponseError("v_versions", response.result);
  }

  return result.data;
}
