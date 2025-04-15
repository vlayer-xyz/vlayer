import { describe, test, expect } from "vitest";
import { validateJrpcResponse } from "./jrpc";

describe("JSON RPC response validators", () => {
  const INVALID_RESPONSE_MSG =
    "Unexpected: response is not a valid JSON RPC response: ";

  test("expects fully specified result response", () => {
    expect(validateJrpcResponse({ id: 1, jsonrpc: "2.0", result: {} })).toEqual(
      { id: 1, jsonrpc: "2.0", result: {} },
    );
    expect(
      validateJrpcResponse({ id: 1, jsonrpc: "2.0", result: { data: {} } }),
    ).toEqual({ id: 1, jsonrpc: "2.0", result: { data: {} } });
    expect(
      validateJrpcResponse({ id: 1, jsonrpc: "2.0", result: "something" }),
    ).toEqual({ id: 1, jsonrpc: "2.0", result: "something" });
    expect(validateJrpcResponse({ id: 1, jsonrpc: "2.0", result: [] })).toEqual(
      { id: 1, jsonrpc: "2.0", result: [] },
    );

    expect(() => validateJrpcResponse({ id: 1, jsonrpc: "2.0" })).toThrowError(
      INVALID_RESPONSE_MSG + JSON.stringify({ id: 1, jsonrpc: "2.0" }),
    );
    expect(() =>
      validateJrpcResponse({
        id: 1,
        jsonrpc: "2.0",
        result: 123,
        error: { code: 123, message: "boom" },
      }),
    ).toThrowError(
      INVALID_RESPONSE_MSG +
        JSON.stringify({
          id: 1,
          jsonrpc: "2.0",
          result: 123,
          error: { code: 123, message: "boom" },
        }),
    );
    expect(() =>
      validateJrpcResponse({ id: 1, jsonrpc: "2.0", error: 123 }),
    ).toThrowError(
      INVALID_RESPONSE_MSG +
        JSON.stringify({ id: 1, jsonrpc: "2.0", error: 123 }),
    );
  });
});
