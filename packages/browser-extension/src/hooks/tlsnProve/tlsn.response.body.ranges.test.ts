import { describe, test, expect } from "vitest";
import { calculateResponseRanges } from "./tlsn.response.ranges";
import {
  RedactResponseJsonBody,
  RedactResponseJsonBodyExcept,
} from "src/web-proof-commons/types/message";

import { ParsedTranscriptData } from "tlsn-js";
import { BodyRangeNotFoundError, PathNotFoundError } from "./tlsn.ranges.error";
import { InvalidPathError, NonStringValueError } from "./tlsn.ranges.error";

describe("calculateResponseRanges", () => {
  describe("json_body redaction", () => {
    test("simple json paths", () => {
      const name = "John";
      const headers =
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
      const raw = headers + `{"name": "${name}", "age": 30}`;
      const transcriptRanges = {
        body: { start: headers.length, end: raw.length },
        headers: {},
      } as ParsedTranscriptData;

      const redactionItem = {
        response: {
          json_body: ["name"],
        },
      } as RedactResponseJsonBody;

      const result = calculateResponseRanges(
        redactionItem,
        raw,
        transcriptRanges,
      );
      expect(result).toEqual([
        {
          start: headers.length + raw.slice(headers.length).indexOf("John"),
          end: headers.length + raw.slice(headers.length).indexOf("John") + 4,
        },
      ]);
    });

    test("multiple json paths", () => {
      const headers =
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
      const raw =
        headers + `{"name": "John", "email": "john@example.com", "age": 30}`;
      const transcriptRanges = {
        body: { start: headers.length, end: raw.length },
        headers: {},
      } as ParsedTranscriptData;

      const redactionItem = {
        response: {
          json_body: ["name", "email"],
        },
      } as RedactResponseJsonBody;

      const result = calculateResponseRanges(
        redactionItem,
        raw,
        transcriptRanges,
      );
      expect(result).toEqual([
        {
          start: headers.length + raw.slice(headers.length).indexOf("John"),
          end: headers.length + raw.slice(headers.length).indexOf("John") + 4,
        },
        {
          start:
            headers.length +
            raw.slice(headers.length).indexOf("john@example.com"),
          end:
            headers.length +
            raw.slice(headers.length).indexOf("john@example.com") +
            16,
        },
      ]);
    });

    test("throws for non-string values", () => {
      const raw = '{"user": {"name": "John", "details": {"age": 30}}}';
      const transcriptRanges = {
        body: { start: 0, end: raw.length },
        headers: {},
      } as ParsedTranscriptData;

      const redactionItem = {
        response: {
          json_body: ["user.details.age"],
        },
      } as RedactResponseJsonBody;

      expect(() =>
        calculateResponseRanges(redactionItem, raw, transcriptRanges),
      ).toThrow(NonStringValueError);
    });

    test("array indices in paths", () => {
      const raw = '{"users": [{"name": "John"}, {"name": "Jane"}]}';
      const transcriptRanges = {
        body: { start: 0, end: raw.length },
        headers: {},
      } as ParsedTranscriptData;

      const redactionItem = {
        response: {
          json_body: ["users[1].name"],
        },
      } as RedactResponseJsonBody;

      const result = calculateResponseRanges(
        redactionItem,
        raw,
        transcriptRanges,
      );
      expect(result).toEqual([
        { start: raw.indexOf("Jane"), end: raw.indexOf("Jane") + 4 },
      ]); // "Jane" position
    });

    test("throws for invalid paths", () => {
      const raw = '{"name": "John"}';
      const transcriptRanges = {
        body: { start: 0, end: raw.length },
        headers: {},
      } as ParsedTranscriptData;

      const redactionItem = {
        response: {
          json_body: ["invalid.path"],
        },
      } as RedactResponseJsonBody;

      expect(() =>
        calculateResponseRanges(redactionItem, raw, transcriptRanges),
      ).toThrow(new PathNotFoundError("invalid.path"));
    });

    test("throws when body range is missing", () => {
      const raw = '{"name": "John"}';
      const transcriptRanges = {
        headers: {},
      } as ParsedTranscriptData;

      const redactionItem = {
        response: {
          json_body: ["name"],
        },
      } as RedactResponseJsonBody;

      expect(() =>
        calculateResponseRanges(redactionItem, raw, transcriptRanges),
      ).toThrow(BodyRangeNotFoundError);
    });

    test("throws for invalid JSON", () => {
      const raw = "{ a: 12, b: 13 }";
      const somePath = "some.path[1";
      const transcriptRanges = {
        body: { start: 0, end: raw.length },
        headers: {},
      } as ParsedTranscriptData;

      const redactionItem = {
        response: {
          json_body: [somePath],
        },
      } as RedactResponseJsonBody;

      expect(() =>
        calculateResponseRanges(redactionItem, raw, transcriptRanges),
      ).toThrow(new InvalidPathError(somePath));
    });
  });

  describe("json_body_except redaction", () => {
    test("string values except specified paths", () => {
      const raw = '{"name": "John", "email": "john@example.com", "age": 30}';
      const email = "john@example.com";
      const transcriptRanges = {
        body: { start: 0, end: raw.length },
        headers: {},
      } as ParsedTranscriptData;

      const redactionItem = {
        response: {
          json_body_except: ["name"],
        },
      } as RedactResponseJsonBodyExcept;

      const result = calculateResponseRanges(
        redactionItem,
        raw,
        transcriptRanges,
      );
      expect(result).toEqual([
        {
          start: raw.indexOf(email),
          end: raw.indexOf(email) + email.length,
        },
      ]);
    });

    test("nested objects with except paths", () => {
      const raw =
        '{"user": {"name": "John", "contact": {"email": "john@example.com", "phone": "123456"}}}';
      const email = "john@example.com";
      const transcriptRanges = {
        body: { start: 0, end: raw.length },
        headers: {},
      } as ParsedTranscriptData;

      const redactionItem = {
        response: {
          json_body_except: ["user.name", "user.contact.phone"],
        },
      } as RedactResponseJsonBodyExcept;

      const result = calculateResponseRanges(
        redactionItem,
        raw,
        transcriptRanges,
      );
      expect(result).toEqual([
        {
          start: raw.indexOf(email),
          end: raw.indexOf(email) + email.length,
        },
      ]);
    });

    test("arrays with except paths", () => {
      const raw =
        '{"users": [{"name": "John", "email": "john@example.com"}, {"name": "Jane", "email": "jane@example.com"}]}';
      const email1 = "john@example.com";
      const name2 = "Jane";
      const transcriptRanges = {
        body: { start: 0, end: raw.length },
        headers: {},
      } as ParsedTranscriptData;

      const redactionItem = {
        response: {
          json_body_except: ["users.0.name", "users.1.email"],
        },
      } as RedactResponseJsonBodyExcept;

      const result = calculateResponseRanges(
        redactionItem,
        raw,
        transcriptRanges,
      );
      expect(result).toEqual([
        {
          start: raw.indexOf(email1),
          end: raw.indexOf(email1) + email1.length,
        },
        { start: raw.indexOf(name2), end: raw.indexOf(name2) + name2.length },
      ]);
    });
  });
});
