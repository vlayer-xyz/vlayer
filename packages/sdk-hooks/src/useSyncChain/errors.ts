export class ChainSwitchError extends Error {
  constructor(chainName: string) {
    super(
      `Failed to switch to ${chainName} make sure you have it in your wallet`,
    );
    this.name = "ChainSwitchError";
  }
}

export class ChainNotSupportedError extends Error {
  constructor(chainName: string) {
    super(`Chain ${chainName} is not supported`);
    this.name = "ChainNotSupportedError";
  }
}

export class MissingConfigChainError extends Error {
  constructor() {
    super("Env chain not defined");
    this.name = "MissingChainError";
  }
}
