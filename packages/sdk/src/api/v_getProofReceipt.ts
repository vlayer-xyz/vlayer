import {
  type VGetProofReceiptParams,
  type VGetProofReceiptResponse,
} from "types/vlayer";
import { parseVCallResponseError } from "./lib/errors";
import { vGetProofReceiptSchema } from "./lib/types/vlayer";
import { Client } from "./utils/JRpcClient";

export async function v_getProofReceipt(
  params: VGetProofReceiptParams,
  url: string = "http://127.0.0.1:3000",
  token?: string,
): Promise<VGetProofReceiptResponse> {
  const client = new Client(url, token);
  const response = await client.send("v_getProofReceipt", params);
  if ("error" in response) {
    throw parseVCallResponseError(
      response.error as { message: string | undefined },
    );
  }
  return vGetProofReceiptSchema.parse(response);
}
