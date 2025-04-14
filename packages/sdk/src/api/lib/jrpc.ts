import { responseSchema, type Response } from "../lib/types/jrpc";
import { JrpcInvalidResponseError } from "./errors";

export function validateJrpcResponse(response: unknown): Response {
  const parsedResponse = responseSchema.safeParse(response);

  if (!parsedResponse.success) {
    throw new JrpcInvalidResponseError(response, parsedResponse.error);
  }

  return parsedResponse.data;
}
