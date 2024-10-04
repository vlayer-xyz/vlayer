export const enum ExtensionAction {
  RequestWebProof,
}

export const enum ExtensionMessage {
  ProofDone,
  ProofError,
}

export type WebProverSessionConfig ={
  notaryUrl: string;
  wsProxyUrl: string;
}
