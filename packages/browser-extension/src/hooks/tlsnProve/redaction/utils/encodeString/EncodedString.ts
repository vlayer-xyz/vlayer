import { EncodingMismatchError } from "../error";
import { encoder, Encoding } from "./Encoding";

export function indexInArray(
  haystack: Uint8Array,
  needle: Uint8Array,
  from: number = 0,
) {
  const haystackLen = haystack.length;
  const needleLen = needle.length;

  if (needleLen === 0) {
    return 0;
  }
  if (needleLen > haystackLen) {
    return -1;
  }

  return (
    Array.from(
      { length: haystackLen - needleLen + 1 },
      (_, i) => i + from,
    ).find((i) => needle.every((byte, j) => haystack[i + j] === byte)) ?? -1
  );
}
export class EncodedString {
  public bytesRepresentation: Uint8Array;
  public stringRepresentation: string;
  constructor(
    stringValue: string,
    public encoding: Encoding,
  ) {
    this.bytesRepresentation = encoder(encoding).encode(stringValue);
    this.stringRepresentation = stringValue;
  }

  indexOf(needle: EncodedString | string, from: number = 0): number {
    if (needle instanceof EncodedString && needle.encoding !== this.encoding) {
      throw new EncodingMismatchError(this.encoding, needle.encoding);
    }
    const needleValue =
      needle instanceof EncodedString
        ? needle.bytesRepresentation
        : encoder(this.encoding).encode(needle);
    return indexInArray(this.bytesRepresentation, needleValue, from);
  }
  nthIndexOf(
    needle: string | EncodedString,
    n: number,
    from: number = 0,
  ): number {
    let count = 0;
    while (count < n) {
      count++;

      const index = this.indexOf(needle, from);
      if (index === -1) {
        return -1;
      }
      from = index + 1;
    }
    return from - 1;
  }

  get length() {
    return this.bytesRepresentation.length;
  }

  split(separator: string | EncodedString): EncodedString[] {
    return this.stringRepresentation
      .split(
        separator instanceof EncodedString
          ? separator.stringRepresentation
          : separator,
      )
      .map((str) => new EncodedString(str, this.encoding));
  }

  includes(needle: string | EncodedString): boolean {
    return this.stringRepresentation.includes(
      needle instanceof EncodedString ? needle.stringRepresentation : needle,
    );
  }

  equals(other: EncodedString) {
    return this.stringRepresentation === other.stringRepresentation;
  }

  toString() {
    return this.stringRepresentation;
  }

  slice(start: number, end: number): EncodedString {
    const slicedBytes = this.bytesRepresentation.slice(start, end);
    return new EncodedString(
      new TextDecoder(this.encoding).decode(slicedBytes),
      this.encoding,
    );
  }

  caseInsensitiveIndexOf(needle: string, index: number = 0): number {
    return new EncodedString(
      this.stringRepresentation.toLowerCase(),
      this.encoding,
    ).indexOf(needle.toLowerCase(), index);
  }
}
