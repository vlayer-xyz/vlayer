import type { z } from "zod";

export class EnvValidationError extends Error {
  constructor(validationResult: z.SafeParseError<unknown>) {
    super(
      "Some environment variables are misconfigured:\n" +
        validationResult.error.errors
          .map((err) => `-${err.path.join(".")}: ${err.message}`)
          .join("\n"),
    );
    this.name = "EnvValidationError";
    Object.setPrototypeOf(this, EnvValidationError.prototype);
  }
}

export class AccountNotSetError extends Error {
  constructor() {
    super("Account is not set, privateKey has to be added to the env");
  }
}
