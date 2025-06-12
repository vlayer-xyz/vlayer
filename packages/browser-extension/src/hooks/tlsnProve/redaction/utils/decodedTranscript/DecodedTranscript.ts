import { Transcript } from "tlsn-js";

export class DecodedTranscript {
  public sent: string;
  public recv: string;
  constructor(transcript: Transcript) {
    this.recv = bytesToUtf8(replaceRedactions(transcript.raw.recv));
    this.sent = bytesToUtf8(replaceRedactions(transcript.raw.sent));
  }
}

function bytesToUtf8(array: number[]): string {
  return Buffer.from(array).toString("utf8");
}

function replaceRedactions(
  array: number[],
  replaceCharacter: string = "*",
): number[] {
  const replaceCharByte = replaceCharacter.charCodeAt(0);
  return array.map((byte) => (byte === 0 ? replaceCharByte : byte));
}
