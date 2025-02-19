import { describe, test, expect } from "vitest";
import {
  RedactResponseJsonBody,
  RedactResponseJsonBodyExcept,
} from "src/web-proof-commons";
import {
  InvalidJsonError,
  PathNotFoundError,
  InvalidPathError,
  NonStringValueError,
  getStringPaths,
  parseHttpMessage,
  EncodedString,
} from "../utils";
import {
  calculateJsonBodyRanges,
  filterExceptPaths,
  validPathRegex,
} from "./tlsn.response.body.ranges";
import { MessageTranscript } from "../types";

const paths = {
  valid: [
    "[2]",
    "key1",
    "key1.key2",
    "key1[3]",
    "[2].key1[5]",
    "key1[1].key2[2].key3",
    "key1.key2[0].key3",
    "key_with_underscore[42].nested_key",
    "_key123[4].key",
  ],
  invalid: [
    "key1..key2",
    "key1[abc]",
    "key1[key2]",
    ".key1",
    "1key",
    "[key]",
    "key1.[2]",
    "key1.key2[]",
    "[123abc]",
    "key1..[3]",
  ],
};
const createTestData = (body: string) => {
  const headers = `HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: ${Buffer.byteLength(body)}\r\n\r\n`;
  const raw = headers + body;
  return parseHttpMessage(raw);
};

const valueRange = (
  transcript: MessageTranscript,
  value: string,
  index: number = 1,
) => ({
  start: transcript.message.content.nthIndexOf(value, index),
  end:
    transcript.message.content.nthIndexOf(value, index) +
    new EncodedString(value, transcript.encoding).length,
});

describe("json body redaction", () => {
  describe("json_body redaction", () => {
    test("simple json paths", () => {
      const name = "José 🌟";
      const transcript = createTestData(`{"name": "${name}", "age": 30}`);

      const redactionItem = {
        response: {
          json_body: ["name"],
        },
      } as RedactResponseJsonBody;

      const result = calculateJsonBodyRanges(
        transcript,
        redactionItem.response.json_body,
      );
      expect(result).toEqual([valueRange(transcript, name)]);
    });

    test("simple json without whitespace at all", () => {
      const transcript = createTestData(`{"iam":"a","ve":{"ry":"valid JSON"}}`);
      const redactionItem = {
        response: {
          json_body: ["ve.ry"],
        },
      } as RedactResponseJsonBody;
      const result = calculateJsonBodyRanges(
        transcript,
        redactionItem.response.json_body,
      );
      expect(result).toEqual([valueRange(transcript, "valid JSON")]);
    });

    test("simple json paths with unnecessary whitespace and newlines in the value", () => {
      const name = "José 🌟";
      const transcript = createTestData(
        `{"name":"${name}","age":30, "im" : {  
                 "very" :{               "nested" : {  
            
           "and" : 
           "ugly"} }   } 
            \n
            \n
            \n
          }`,
      );

      const redactionItem = {
        response: {
          json_body: ["im.very.nested.and"],
        },
      } as RedactResponseJsonBody;

      const result = calculateJsonBodyRanges(
        transcript,
        redactionItem.response.json_body,
      );
      expect(result).toEqual([valueRange(transcript, "ugly")]);
    });

    test("multiple json paths", () => {
      const name = "María 👩";
      const email = "maría@例子.com";
      const transcript = createTestData(
        `{"name": "${name}", "email": "${email}", "age": 30}`,
      );

      const redactionItem = {
        response: {
          json_body: ["name", "email"],
        },
      } as RedactResponseJsonBody;

      const result = calculateJsonBodyRanges(
        transcript,
        redactionItem.response.json_body,
      );
      expect(result).toEqual([
        valueRange(transcript, name),
        valueRange(transcript, email),
      ]);
    });

    test("throws for non-string values", () => {
      const transcript = createTestData(
        '{"user": {"name": "José 🌟", "details": {"age": 30}}}',
      );

      const redactionItem = {
        response: {
          json_body: ["user.details.age"],
        },
      } as RedactResponseJsonBody;

      expect(() =>
        calculateJsonBodyRanges(transcript, redactionItem.response.json_body),
      ).toThrow(NonStringValueError);
    });

    test("array indices in paths", () => {
      const name = "María 👩";
      const transcript = createTestData(
        `{"users": [{"name": "José 🌟"}, {"name": "${name}"}]}`,
      );

      const redactionItem = {
        response: {
          json_body: ["users[1].name"],
        },
      } as RedactResponseJsonBody;

      const result = calculateJsonBodyRanges(
        transcript,
        redactionItem.response.json_body,
      );
      expect(result).toEqual([valueRange(transcript, name)]);
    });

    test("simple array of fruits and numbers", () => {
      const orange = "🍊";
      const transcript = createTestData('["🍊","🍎", "🍌", "🍊",1,"🍐"]');

      const redactionItem = {
        response: {
          json_body: ["[3]"],
        },
      } as RedactResponseJsonBody;

      const result = calculateJsonBodyRanges(
        transcript,
        redactionItem.response.json_body,
      );
      expect(result).toEqual([valueRange(transcript, orange, 2)]);
    });

    test("simple array of strings and numbers with number at path", () => {
      const transcript = createTestData('["🍎", "🍌", "🍊",1,"🍐"]');

      const redactionItem = {
        response: {
          json_body: ["[3]"],
        },
      } as RedactResponseJsonBody;

      expect(() => {
        calculateJsonBodyRanges(transcript, redactionItem.response.json_body);
      }).toThrow(NonStringValueError);
    });

    test("deeply nested value in path", () => {
      const idValue = "测试值🔑";
      const transcript = createTestData(
        `{"data": [{"users": [{"settings": [{"config": {"features": [{"enabled": true, "id": "${idValue}"}]}}]}]}]}`,
      );

      const redactionItem = {
        response: {
          json_body: ["data[0].users[0].settings[0].config.features[0].id"],
        },
      } as RedactResponseJsonBody;

      const result = calculateJsonBodyRanges(
        transcript,
        redactionItem.response.json_body,
      );
      expect(result).toEqual([valueRange(transcript, idValue)]);
    });

    test("fails on deeply nested boolean value in path", () => {
      const transcript = createTestData(
        '{"data": [{"users": [{"settings": [{"config": {"features": [{"enabled": true, "id": "测试🔑"}]}}]}]}]}',
      );

      const redactionItem = {
        response: {
          json_body: [
            "data[0].users[0].settings[0].config.features[0].enabled",
          ],
        },
      } as RedactResponseJsonBody;

      expect(() =>
        calculateJsonBodyRanges(transcript, redactionItem.response.json_body),
      ).toThrow(NonStringValueError);
    });

    test("throws for invalid paths", () => {
      const transcript = createTestData('{"name": "José 🌟"}');

      const redactionItem = {
        response: {
          json_body: ["invalid.path"],
        },
      } as RedactResponseJsonBody;

      expect(() =>
        calculateJsonBodyRanges(transcript, redactionItem.response.json_body),
      ).toThrow(new PathNotFoundError("invalid.path"));
    });

    test("throws for invalid JSON", () => {
      const transcript = createTestData("{a: 12, b: 13}");

      const redactionItem = {
        response: {
          json_body: ["some.path[1]"],
        },
      } as RedactResponseJsonBody;

      expect(() =>
        calculateJsonBodyRanges(transcript, redactionItem.response.json_body),
      ).toThrow(InvalidJsonError);
    });

    test("throws for invalid path", () => {
      const transcript = createTestData('{"name": "José 🌟"}');

      const redactionItem = {
        response: {
          json_body: ["some.path[1"],
        },
      } as RedactResponseJsonBody;

      expect(() =>
        calculateJsonBodyRanges(transcript, redactionItem.response.json_body),
      ).toThrow(new InvalidPathError("some.path[1"));
    });
  });

  describe("json_body_except redaction", () => {
    test("string values except specified paths", () => {
      const email = "josé@例子.com";
      const transcript = createTestData(
        `{"name": "José 🌟", "email": "${email}", "age": 30}`,
      );

      const redactionItem = {
        response: {
          json_body_except: ["name"],
        },
      } as RedactResponseJsonBodyExcept;
      const paths = getStringPaths(transcript.body.content.toString());
      const filteredPaths = filterExceptPaths(
        redactionItem.response.json_body_except,
        paths,
      );
      const result = calculateJsonBodyRanges(transcript, filteredPaths);
      expect(result).toEqual([valueRange(transcript, email)]);
    });

    test("nested objects with except paths", () => {
      const email = "josé@例子.com";
      const transcript = createTestData(
        `{"user": {"name": "José 🌟", "contact": {"email": "${email}", "phone": "123456"}}}`,
      );

      const redactionItem = {
        response: {
          json_body_except: ["user.name", "user.contact.phone"],
        },
      } as RedactResponseJsonBodyExcept;
      const paths = getStringPaths(transcript.body.content.toString());
      const filteredPaths = filterExceptPaths(
        redactionItem.response.json_body_except,
        paths,
      );
      const result = calculateJsonBodyRanges(transcript, filteredPaths);
      expect(result).toEqual([valueRange(transcript, email)]);
    });

    test("arrays with except paths", () => {
      const email1 = "josé@例子.com";
      const name2 = "María 👩";
      const transcript = createTestData(
        `{"users": [{"name": "José 🌟", "email": "${email1}"}, {"name": "${name2}", "email": "maría@例子.com"}]}`,
      );

      const redactionItem = {
        response: {
          json_body_except: ["users.0.name", "users.1.email"],
        },
      } as RedactResponseJsonBodyExcept;

      const paths = getStringPaths(transcript.body.content.toString());
      const filteredPaths = filterExceptPaths(
        redactionItem.response.json_body_except,
        paths,
      );
      const result = calculateJsonBodyRanges(transcript, filteredPaths);
      expect(result).toEqual([
        valueRange(transcript, email1),
        valueRange(transcript, name2),
      ]);
    });

    test("arrays with except paths", () => {
      const name1 = "José 🌟";
      const name2 = "María 👩";
      const transcript = createTestData(
        `{"users": [{"name": "${name1}", "email": "josé@例子.com"}, {"name": "${name2}", "email": "maría@例子.com"}]}`,
      );

      const redactionItem = {
        response: {
          json_body_except: ["users.0.email", "users.1.email"],
        },
      } as RedactResponseJsonBodyExcept;

      const paths = getStringPaths(transcript.body.content.toString());
      const filteredPaths = filterExceptPaths(
        redactionItem.response.json_body_except,
        paths,
      );
      const result = calculateJsonBodyRanges(transcript, filteredPaths);
      expect(result).toEqual([
        valueRange(transcript, name1),
        valueRange(transcript, name2),
      ]);
    });
  });
});

describe("validPathRegex", () => {
  test("valid paths", () => {
    expect(paths.valid.every((path: string) => validPathRegex.test(path))).toBe(
      true,
    );
  });

  test("invalid paths", () => {
    expect(
      paths.invalid.every((path: string) => !validPathRegex.test(path)),
    ).toBe(true);
  });
});
