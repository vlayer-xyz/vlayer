import { CommitData } from "tlsn-js/src/types";
import { Encoding } from "./encodeString/Encoding";

export class InvalidEncodingError extends Error {
  constructor(encoding: string) {
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

export class RedactionError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "RedactionError";
  }
}

export class PathNotFoundError extends RedactionError {
  constructor(path: string) {
    super(`Path ${path} not found in JSON body`);
    this.name = "PathNotFoundError";
  }
}

export class BodyRangeNotFoundError extends RedactionError {
  constructor() {
    super("Body range not found");
    this.name = "BodyRangeNotFoundError";
  }
}

export class NonStringValueError extends RedactionError {
  constructor(value: string) {
    super(`Non-string value found: ${value}`);
    this.name = "NonStringValueError";
  }
}

export class InvalidPathError extends RedactionError {
  constructor(path: string) {
    super(`Invalid path: ${path}`);
    this.name = "InvalidPathError";
  }
}

export class InvalidJsonError extends RedactionError {
  constructor(message: string) {
    super(`Invalid JSON: ${message}`);
    this.name = "InvalidJsonError";
  }
}

export class OutOfBoundsError extends RedactionError {
  constructor(range: CommitData) {
    super(`Range ${range.start} - ${range.end} is out of bounds`);
    this.name = "OutOfBoundsError";
  }
}

export class InvalidRangeError extends RedactionError {
  constructor(range: CommitData) {
    super(`Range ${range.start} - ${range.end} is invalid`);
    this.name = "InvalidRangeError";
  }
}

export class InvalidHttpStringError extends RedactionError {
  constructor() {
    super("Invalid HTTP request string: No header-body delimiter found.");
    this.name = "InvalidHttpStringError";
  }
}

export class InvalidHttpMessageError extends RedactionError {
  constructor(message: string) {
    super(`Invalid HTTP message: ${message}`);
    this.name = "InvalidHttpMessageError";
  }
}

export class HeaderNotFoundError extends RedactionError {
  constructor(header: string) {
    super(`Header ${header} not found in transcript`);
    this.name = "HeaderNotFoundError";
  }
}

export class NoGivenParamInUrlError extends RedactionError {
  constructor(param: string) {
    super(`No given param in url: ${param}`);
    this.name = "NoGivenParamInUrlError";
  }
}
