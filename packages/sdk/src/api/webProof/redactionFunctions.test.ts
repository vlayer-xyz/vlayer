import type {
  RedactRequestHeaders,
  RedactRequestHeadersExcept,
  RedactRequestUrlQueryParam,
  RedactRequestUrlQueryParamExcept,
  RedactResponseHeaders,
  RedactResponseHeadersExcept,
  RedactResponseJsonBody,
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

  it("should redact all request headers except", () => {
    const redacted = request.headers.redactAllExcept(["Authorization"]);
    expectTypeOf(redacted).toEqualTypeOf<RedactRequestHeadersExcept>();
    expect(redacted).toEqual({
      request: {
        headers_except: ["Authorization"],
      },
    });
  });

  it("should redact request url query params", () => {
    const redacted = request.url.redactQueryParams(["token"]);
    expectTypeOf(redacted).toEqualTypeOf<RedactRequestUrlQueryParam>();
    expect(redacted).toEqual({
      request: {
        url_query: ["token"],
      },
    });
  });

  it("should redact all request url query params except", () => {
    const redacted = request.url.redactAllQueryParamsExcept(["token"]);
    expectTypeOf(redacted).toEqualTypeOf<RedactRequestUrlQueryParamExcept>();
    expect(redacted).toEqual({
      request: {
        url_query_except: ["token"],
      },
    });
  });

  it("should redact response headers", () => {
    const redacted = response.headers.redact(["Authorization"]);
    expectTypeOf(redacted).toEqualTypeOf<RedactResponseHeaders>();
    expect(redacted).toEqual({
      response: {
        headers: ["Authorization"],
      },
    });
  });

  it("should redact all response headers except", () => {
    const redacted = response.headers.redactAllExcept(["Authorization"]);
    expectTypeOf(redacted).toEqualTypeOf<RedactResponseHeadersExcept>();
    expect(redacted).toEqual({
      response: {
        headers_except: ["Authorization"],
      },
    });
  });

  it("should redact response jsonBody fields", () => {
    const redacted = response.jsonBody.redact(["screen_name"]);
    expectTypeOf(redacted).toEqualTypeOf<RedactResponseJsonBody>();
    expect(redacted).toEqual({
      response: {
        json_body: ["screen_name"],
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
