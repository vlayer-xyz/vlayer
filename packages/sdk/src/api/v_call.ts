import {
  type CallContext,
  type CallParams,
  type VCallResponse,
} from "types/vlayer";
import { parseVCallResponseError } from "./lib/errors";
import { Client } from "./utils/JRpcClient";

export async function v_call(
  call: CallParams,
  context: CallContext,
  url: string = "http://127.0.0.1:3000",
  token?: string,
): Promise<VCallResponse> {
  const client = new Client(url, token);
  const response = await client.send("v_call", [call, context]);
  if ("error" in response) {
    throw parseVCallResponseError(
      response.error as { message: string | undefined },
    );
  }
  return response as Promise<VCallResponse>;
}
