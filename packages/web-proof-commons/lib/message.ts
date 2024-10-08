export const enum ExtensionAction {
  RequestWebProof,
}

export const enum ExtensionMessageType {
  ProofDone,
  ProofError,
  RedirectBack,
}

export type ExtensionMessage =
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  | { type: ExtensionMessageType.ProofDone; proof: any } // Change to WebProof
  | { type: ExtensionMessageType.ProofError; error: string }
  | { type: ExtensionMessageType.RedirectBack };

export type WebProverSessionConfig = {
  notaryUrl: string;
  wsProxyUrl: string;
};
