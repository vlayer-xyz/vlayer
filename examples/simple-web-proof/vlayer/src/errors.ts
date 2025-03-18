export class AlreadyMintedError extends Error {
  constructor() {
    super("Already minted");
    this.name = "AlreadyMintedError";
  }
}
