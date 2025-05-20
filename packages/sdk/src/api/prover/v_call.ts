import {
  type CallContext,
  type CallParams,
  type CallHash,
  callHashSchema,
} from "types/vlayer";
import {
  handleProverResponseError,
  handleAuthErrors,
} from "../lib/handleErrors";
import { InvalidProverResponseError } from "../lib/errors";
import { validateJrpcResponse } from "../lib/jrpc";
import debug from "debug";

const log = debug("vlayer:v_call");

function v_callBody(call: CallParams, context: CallContext) {
  log("call", call);
  log("context", context);
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
  token?: string,
): Promise<CallHash> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  if (token !== undefined) {
    headers["Authorization"] = "Bearer " + token;
  }

  const rawResponse = await fetch(url, {
    method: "POST",
    body: JSON.stringify(v_callBody(call, context)),
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

  const result = callHashSchema.safeParse(response.result);
  if (!result.success) {
    throw new InvalidProverResponseError("v_call", response.result);
  }

  return result.data;
}
