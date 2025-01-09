import type {
  RedactRequestHeaders,
  RedactResponseJsonBodyExcept,
} from "src/web-proof-commons";
import { describe, expect, expectTypeOf, it } from "vitest";
import { request, response } from "./redactionFunctions";

describe("redactionFunctions", () => {
  it("should redact request headers", () => {
    const redacted = request.headers.redact(["Authorization"]);
    expectTypeOf(redacted).toEqualTypeOf<RedactRequestHeaders>();
    expect(redacted).toEqual({
      request: {
        headers: ["Authorization"],
      },
    });
  });

  it("should redact all response jsonBody fields except", () => {
    const redacted = response.jsonBody.redactAllExcept(["screen_name"]);
    expectTypeOf(redacted).toEqualTypeOf<RedactResponseJsonBodyExcept>();
    expect(redacted).toEqual({
      response: {
        json_body_except: ["screen_name"],
      },
    });
  });
});
