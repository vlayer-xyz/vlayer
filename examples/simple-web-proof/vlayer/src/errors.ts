export class KnownAppError extends Error {
  constructor(name: string, message: string) {
    super(message);
    this.name = name;
  }
}
export class AlreadyMintedError extends KnownAppError {
  constructor() {
    super(
      "AlreadyMintedError",
      "NFT has already been minted for this account.",
    );
  }
}

export class FaucetError extends KnownAppError {
  constructor() {
    super("FaucetError", "Failed to fund account.");
  }
}
