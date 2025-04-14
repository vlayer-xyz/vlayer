import { describe, test, expect } from "vitest";
import { handleAuthErrors, handleProverResponseError } from "./handleErrors";
import {
  VersionError,
  HttpAuthorizationError,
  HttpUnexpectedError,
} from "./errors";

describe("authorization errors", () => {
  test("authorization error always returns payload", () => {
    expect(handleAuthErrors(401, { error: "Invalid JWT token" })).toEqual(
      new HttpAuthorizationError("Invalid JWT token"),
    );
    expect(handleAuthErrors(401, {})).toEqual(new HttpUnexpectedError(401));
    expect(handleAuthErrors(401)).toEqual(new HttpUnexpectedError(401));
  });

  test("every other error is currently classified as unuexpected", () => {
    expect(handleAuthErrors(403, { error: "Invalid JWT token" })).toEqual(
      new HttpUnexpectedError(403),
    );
    expect(handleAuthErrors(403)).toEqual(new HttpUnexpectedError(403));
  });
});

describe("prover errors", () => {
  test("unsupported CallGuestId is reported as mismatched version", () => {
    expect(
      handleProverResponseError({ message: "Unsupported CallGuestID" }),
    ).toBeInstanceOf(VersionError);
  });
  test("every other error is reported as-is", () => {
    expect(
      handleProverResponseError({ message: "revert: some other scary reason" }),
    ).toEqual(new Error("Error response: revert: some other scary reason"));
    expect(handleProverResponseError({ message: undefined })).toEqual(
      new Error("Error response: unknown error"),
    );
  });
});
