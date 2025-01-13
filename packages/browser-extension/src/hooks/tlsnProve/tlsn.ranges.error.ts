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
  constructor() {
    super("Invalid JSON");
    this.name = "InvalidJsonError";
  }
}
