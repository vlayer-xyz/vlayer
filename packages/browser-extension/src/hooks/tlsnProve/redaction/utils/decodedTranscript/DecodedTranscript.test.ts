import { describe, expect, test } from "vitest";
import { Transcript } from "tlsn-js";
import { DecodedTranscript } from "./DecodedTranscript";

// hello world
const TEXT = [0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64];
// kąsać
const UNICODE_TEXT = [0x6b, 0xc4, 0x85, 0x73, 0x61, 0xc4, 0x87];

// "recv: "
const RECV = [0x72, 0x65, 0x63, 0x76, 0x3a, 0x20];
// "sent: "
const SENT = [0x73, 0x65, 0x6e, 0x74, 0x3a, 0x20];

const transcript = (text: number[]) =>
  new Transcript({ sent: [...SENT, ...text], recv: [...RECV, ...text] });

describe("DecodedTranscript", () => {
  describe("plain ascii text", () => {
    test("returns valid strings if no redaction made", () => {
      expect(new DecodedTranscript(transcript(TEXT))).toMatchObject({
        recv: "recv: hello world",
        sent: "sent: hello world",
      });
    });

    test("returns valid strings if redaction made", () => {
      expect(
        new DecodedTranscript(transcript([0, ...TEXT, 0, 0])),
      ).toMatchObject({
        recv: "recv: *hello world**",
        sent: "sent: *hello world**",
      });
    });
  });

  describe("unicode text", () => {
    test("returns valid strings if no redaction made", () => {
      expect(new DecodedTranscript(transcript(UNICODE_TEXT))).toMatchObject({
        recv: "recv: kąsać",
        sent: "sent: kąsać",
      });
    });
    test("returns valid strings if redaction made", () => {
      expect(
        new DecodedTranscript(transcript([0, ...UNICODE_TEXT, 0, 0])),
      ).toMatchObject({
        recv: "recv: *kąsać**",
        sent: "sent: *kąsać**",
      });
    });
  });
});
