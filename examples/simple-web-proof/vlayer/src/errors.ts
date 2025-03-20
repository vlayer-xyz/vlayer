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
