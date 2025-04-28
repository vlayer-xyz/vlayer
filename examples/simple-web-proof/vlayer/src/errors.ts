export class AppError extends Error {
  constructor(name: string, message: string) {
    super(message);
    this.name = name;
  }
}
export class AlreadyMintedError extends AppError {
  constructor() {
    super(
      "AlreadyMintedError",
      "NFT has already been minted for this account.",
    );
  }
}

export class FaucetError extends AppError {
  constructor() {
    super("FaucetError", "Failed to fund account.");
  }
}

export class UseExtensionError extends AppError {
  constructor(message: string) {
    super("UseExtensionError", message);
  }
}

export class WebProofError extends AppError {
  constructor(message: string) {
    super("WebProofError", message);
  }
}

export class UseChainError extends AppError {
  constructor(message: string) {
    super("UseChainError", message);
  }
}
