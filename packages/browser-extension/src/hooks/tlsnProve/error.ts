import { Encoding } from "./utils/encodeString/Encoding";

export class InvalidEncodingError extends Error {
  constructor(encoding: Encoding) {
    super(
      `Invalid encoding: ${encoding} only ${Object.values(Encoding).join(", ")} are supported`,
    );
  }
}

export class EncodingMismatchError extends Error {
  constructor(encoding: Encoding, needleEncoding: Encoding) {
    super(`Encoding mismatch: ${encoding} and ${needleEncoding}`);
  }
}
