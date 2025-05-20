import { type CallHash } from "types/vlayer";
import {
  handleProverResponseError,
  handleAuthErrors,
} from "../lib/handleErrors";
import { InvalidProverResponseError } from "../lib/errors";
import { proofReceiptSchema, type ProofReceipt } from "../lib/types/vlayer";
import { validateJrpcResponse } from "../lib/jrpc";
import debug from "debug";

const log = debug("vlayer:v_getProofReceipt");

function v_getProofReceiptBody(hash: CallHash) {
  return {
    method: "v_getProofReceipt",
    params: { hash },
    id: 1,
    jsonrpc: "2.0",
  };
}

export async function v_getProofReceipt(
  hash: CallHash,
  url: string = "http://127.0.0.1:3000",
  token?: string,
): Promise<ProofReceipt> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  if (token !== undefined) {
    headers["Authorization"] = "Bearer " + token;
  }
  const rawResponse = await fetch(url, {
    method: "POST",
    body: JSON.stringify(v_getProofReceiptBody(hash)),
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

  const result = proofReceiptSchema.safeParse(response.result);
  if (!result.success) {
    throw new InvalidProverResponseError("v_getProofReceipt", response.result);
  }

  return result.data;
}
