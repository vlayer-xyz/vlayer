export function utf8IndexOf(
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

export class Utf8String {
  private value: Uint8Array;
  private utf16String: string;

  constructor(stringValue: string) {
    this.value = new TextEncoder().encode(stringValue);
    this.utf16String = stringValue;
  }

  indexOf(needle: Utf8String | string, from: number = 0): number {
    const needleValue =
      needle instanceof Utf8String
        ? needle.value
        : new TextEncoder().encode(needle);
    return utf8IndexOf(this.value, needleValue, from);
  }
  nthIndexOf(needle: string, n: number, from: number = 0): number {
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
    return this.value.length;
  }

  split(separator: string | Utf8String): Utf8String[] {
    return this.utf16String
      .split(
        separator instanceof Utf8String ? separator.utf16String : separator,
      )
      .map((str) => new Utf8String(str));
  }

  includes(needle: string | Utf8String): boolean {
    return this.utf16String.includes(
      needle instanceof Utf8String ? needle.utf16String : needle,
    );
  }

  equals(other: Utf8String) {
    return this.utf16String === other.utf16String;
  }

  toUtf16String() {
    return this.utf16String;
  }

  slice(start: number, end: number) {
    const slicedArray = this.value.slice(start, end);
    const decodedString = new TextDecoder().decode(slicedArray);
    return new Utf8String(decodedString);
  }
}
