import { Encoding } from "./Encoding";

import { describe, expect, test } from "vitest";
import { EncodedString } from "./EncodedString";

const testCases = [
  {
    input: "hello",
    expected: {
      utf8: new Uint8Array([104, 101, 108, 108, 111]),
      utf16: new Uint8Array([0, 104, 0, 101, 0, 108, 0, 108, 0, 111]),
    },
    description: "basic ASCII",
  },
  {
    input: "héllo",
    expected: {
      utf8: new Uint8Array([104, 195, 169, 108, 108, 111]),
      utf16: new Uint8Array([0, 104, 0, 233, 0, 108, 0, 108, 0, 111]),
    },
    description: "special characters",
  },
  {
    input: "hello 👋",
    expected: {
      utf8: new Uint8Array([104, 101, 108, 108, 111, 32, 240, 159, 145, 139]),
      utf16: new Uint8Array([
        0, 104, 0, 101, 0, 108, 0, 108, 0, 111, 0, 32, 216, 61, 220, 75,
      ]),
    },
    description: "emoji",
  },
  {
    input: "hello world",
    expected: {
      utf8: [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100],
      utf16: [
        0, 104, 0, 101, 0, 108, 0, 108, 0, 111, 0, 32, 0, 119, 0, 111, 0, 114,
        0, 108, 0, 100,
      ],
    },
    description: "multiple words",
  },
  {
    input: "The quick brown fox jumps över the lazy dog 🦊",
    expected: {
      utf8: [
        84, 104, 101, 32, 113, 117, 105, 99, 107, 32, 98, 114, 111, 119, 110,
        32, 102, 111, 120, 32, 106, 117, 109, 112, 115, 32, 195, 182, 118, 101,
        114, 32, 116, 104, 101, 32, 108, 97, 122, 121, 32, 100, 111, 103, 32,
        240, 159, 166, 138,
      ],
      utf16: [
        0, 84, 0, 104, 0, 101, 0, 32, 0, 113, 0, 117, 0, 105, 0, 99, 0, 107, 0,
        32, 0, 98, 0, 114, 0, 111, 0, 119, 0, 110, 0, 32, 0, 102, 0, 111, 0,
        120, 0, 32, 0, 106, 0, 117, 0, 109, 0, 112, 0, 115, 0, 32, 0, 246, 0,
        118, 0, 101, 0, 114, 0, 32, 0, 116, 0, 104, 0, 101, 0, 32, 0, 108, 0,
        97, 0, 122, 0, 121, 0, 32, 0, 100, 0, 111, 0, 103, 0, 32, 216, 62, 221,
        138,
      ],
    },
    description: "long text with special characters and emoji",
  },
];

describe("Encoding", () => {
  describe("UTF-8", () => {
    testCases.forEach(({ input, expected, description }) => {
      test(`correctly encodes strings - ${description}`, () => {
        const encodedString = new EncodedString(input, Encoding.UTF8);
        expect([...encodedString.bytesRepresentation]).toMatchObject(
          expected.utf8,
        );
        expect(
          new TextDecoder("utf-8").decode(encodedString.bytesRepresentation),
        ).toEqual(input);
      });
    });
  });

  describe("UTF-16", () => {
    testCases.forEach(({ input, expected, description }) => {
      test(`correctly encodes strings - ${description}`, () => {
        const encodedString = new EncodedString(input, Encoding.UTF16);
        expect([...encodedString.bytesRepresentation]).toMatchObject(
          expected.utf16,
        );
        expect(
          new TextDecoder("utf-16be").decode(encodedString.bytesRepresentation),
        ).toEqual(input);
      });
    });
  });
});
