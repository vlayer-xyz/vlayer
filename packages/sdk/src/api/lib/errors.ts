import { ZodError } from "zod";

export class VersionError extends Error {
  constructor(message: string) {
    super(`${message}
    vlayer uses the daily release cycle, and SDK version must match the proving server version.
    Please run "vlayer update" to update the SDK to the latest version.`);
    this.name = "VersionError";
  }
}

export class HttpAuthorizationError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "HttpAuthorizationError";
  }
}

export class HttpUnexpectedError extends Error {
  constructor(code: number) {
    super(`Unexpected: received HTTP response with status code: ${code}`);
    this.name = "HttpUnexpectedError";
  }
}

export class InvalidProverResponseError extends Error {
  constructor(method: string, response: unknown) {
    super(
      `Unexpected: ${method} response is not valid: ${JSON.stringify(response)}`,
    );
    this.name = "InvalidProverResponse";
  }
}

export class JrpcInvalidResponseError extends Error {
  constructor(response: unknown, error: ZodError) {
    super(
      `Unexpected: response is not a valid JSON RPC response: ${JSON.stringify(response)}
${JSON.stringify(error.format(), null, 2)}`,
    );
    this.name = "JrpcInvalidResponseError";
  }
}

export const VLAYER_ERROR_NOTES = {
  [HttpAuthorizationError.name]:
    ", go to https://dashboard.vlayer.xyz to generate a JWT token and save it to VLAYER_API_TOKEN env var",
};

export function httpAuthorizationErrorWithNote(
  error: HttpAuthorizationError,
): HttpAuthorizationError {
  return new HttpAuthorizationError(
    `${error.message}${VLAYER_ERROR_NOTES[error.name]}`,
  );
}
