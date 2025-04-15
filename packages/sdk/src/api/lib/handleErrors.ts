import {
  errorSchema as authErrorSchema,
  type Error as AuthError,
} from "../lib/types/auth";
import {
  HttpAuthorizationError,
  HttpUnexpectedError,
  VersionError,
} from "./errors";
import { StatusCodes } from "http-status-codes";
import { match } from "ts-pattern";

export function handleProverResponseError({
  message,
}: {
  message: string | undefined;
}): Error {
  if (message?.startsWith("Unsupported CallGuestID")) {
    return new VersionError(message);
  }
  return new Error(`Error response: ${message ?? "unknown error"}`);
}

export function handleAuthErrors(code: number, body?: unknown): Error {
  return match(code)
    .with(StatusCodes.UNAUTHORIZED, (code) => {
      const parsedError = authErrorSchema.safeParse(body);
      if (parsedError.success) {
        const authError: AuthError = parsedError.data;
        return new HttpAuthorizationError(authError.error);
      } else {
        return new HttpUnexpectedError(code);
      }
    })
    .otherwise((code) => new HttpUnexpectedError(code));
}
