class BaseError<T extends string> extends Error {
  readonly name: T;

  constructor({ message, name }: { message: string; name: T }) {
    super();
    this.name = name;

    this.message = message;
  }
}

type TlsnProveErrorName =
  | "TLSN_PROVE_ERROR"
  | "TLSN_PROVE_NON_200_RESPONSE_ERROR";

export class TlsnProveError extends BaseError<TlsnProveErrorName> {}

export class TlsnProveNon200ResponseError extends TlsnProveError {
  constructor() {
    super({
      message: `Non 200 response from proven endpoint.`,
      name: "TLSN_PROVE_NON_200_RESPONSE_ERROR",
    });
  }
}
