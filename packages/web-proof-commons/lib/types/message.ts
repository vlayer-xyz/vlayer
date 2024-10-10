import type { Branded } from "../utils.ts";
import type { WebProof } from "./webProof.ts";

export const EXTENSION_STEP = {
  expectUrl: "expectUrl",
  startPage: "startPage",
  notarize: "notarize",
} as const;

export type ExtensionStep =
  (typeof EXTENSION_STEP)[keyof typeof EXTENSION_STEP];

export const enum ExtensionAction {
  RequestWebProof,
}

export type MessageToExtension = {
  action: ExtensionAction;
  payload: WebProverSessionConfig;
};

export const enum ExtensionMessageType {
  ProofDone = "ProofDone",
  ProofError = "ProofError",
  RedirectBack = "RedirectBack",
}

export type ExtensionMessage =
  | { type: ExtensionMessageType.ProofDone; proof: WebProof }
  | { type: ExtensionMessageType.ProofError; error: string }
  | { type: ExtensionMessageType.RedirectBack };

export type WebProverSessionConfig = {
  notaryUrl: string;
  wsProxyUrl: string;
  logoUrl: string;
  steps: WebProofStep[];
};

export type WebProofStep =
  | WebProofStepNotarize
  | WebProofStepExpectUrl
  | WebProofStepStartPage;

export type WebProofStepNotarize = Branded<
  {
    url: string;
    method: string;
    label: string;
    step: typeof EXTENSION_STEP.notarize;
  },
  "notarize"
>;

export type WebProofStepExpectUrl = Branded<
  {
    url: string;
    label: string;
    step: typeof EXTENSION_STEP.expectUrl;
  },
  "expectUrl"
>;

export type WebProofStepStartPage = Branded<
  {
    url: string;
    label: string;
    step: typeof EXTENSION_STEP.startPage;
  },
  "startPage"
>;
