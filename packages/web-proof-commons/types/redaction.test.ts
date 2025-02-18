import { describe, test, expect } from "vitest";
import { RedactionItemsArray } from "./redaction";
describe("RedactionConfig", () => {
  test("accepts empty array", () => {
    const testEmpty = RedactionItemsArray.parse([]);
    expect(testEmpty).toBeDefined();
  });

  test("valid redaction config", () => {
    const testValid = RedactionItemsArray.parse([
      {
        request: {
          headers: ["Authorization"],
        },
      },
      {
        response: {
          headers: ["Content-Type"],
        },
      },
    ]);
    expect(testValid).toBeDefined();
  });

  test("invalid when using both request headers and headers_except", () => {
    expect(() => {
      RedactionItemsArray.parse([
        {
          request: {
            headers: ["Authorization"],
          },
        },
        {
          request: {
            headers_except: ["Authorization"],
          },
        },
      ]);
    }).toThrow("Cannot have both request headers and request headers_except");
  });

  test("invalid when using both response headers and headers_except", () => {
    expect(() => {
      RedactionItemsArray.parse([
        {
          response: {
            headers: ["Content-Type"],
          },
        },
        {
          response: {
            headers_except: ["Authorization"],
          },
        },
      ]);
    }).toThrow("Cannot have both response headers and response headers_except");
  });

  test("invalid when using both request url_query and url_query_except", () => {
    expect(() => {
      RedactionItemsArray.parse([
        {
          request: {
            url_query: ["page"],
          },
        },
        {
          request: {
            url_query_except: ["limit"],
          },
        },
      ]);
    }).toThrow(
      "Cannot have both request url_query and request url_query_except",
    );
  });

  test("invalid when using both response json_body and json_body_except", () => {
    expect(() => {
      RedactionItemsArray.parse([
        {
          response: {
            json_body: ["data"],
          },
        },
        {
          response: {
            json_body_except: ["metadata"],
          },
        },
      ]);
    }).toThrow(
      "Cannot have both response json_body and response json_body_except",
    );
  });
});
