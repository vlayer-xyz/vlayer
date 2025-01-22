import { EncodedString } from "hooks/tlsnProve/redaction/utils/encodeString/EncodedString";
import { expect } from "vitest";

function isEncodedStringComparator(a: unknown): a is EncodedString {
  return a instanceof EncodedString;
}

function areEncodedStringsEqual(a: unknown, b: unknown): boolean | undefined {
  const isAEncodedString = isEncodedStringComparator(a);
  const isBEncodedString = isEncodedStringComparator(b);

  if (isAEncodedString && isBEncodedString) {
    return a.equals(b);
  } else if (isAEncodedString === isBEncodedString) {
    return undefined;
  } else {
    return false;
  }
}

expect.addEqualityTesters([areEncodedStringsEqual]);
