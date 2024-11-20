import type { Branded } from "../utils";
import type { WebProof } from "./webProof";

export const EXTENSION_STEP = {
  expectUrl: "expectUrl",
  startPage: "startPage",
  notarize: "notarize",
} as const;

export type ExtensionStep =
  (typeof EXTENSION_STEP)[keyof typeof EXTENSION_STEP];

export const enum ExtensionAction {
  RequestWebProof = "RequestWebProof",
  NotifyZkProvingStatus = "NotifyZkProvingStatus",
}

export enum ZkProvingStatus {
  NotStarted = "NotStarted",
  Proving = "Proving",
  Done = "Done",
  Error = "Error",
}

export type MessageToExtension =
  | {
      action: ExtensionAction.RequestWebProof;
      payload: WebProverSessionConfig;
    }
  | {
      action: ExtensionAction.NotifyZkProvingStatus;
      payload: {
        status: ZkProvingStatus;
      };
    };

export enum ExtensionMessageType {
  ProofDone = "ProofDone",
  ProofError = "ProofError",
  RedirectBack = "RedirectBack",
  TabOpened = "TabOpened",
  ProofProcessing = "ProofProcessing",
}

export type ExtensionMessage =
  | { type: ExtensionMessageType.ProofDone; payload: { proof: WebProof } }
  | { type: ExtensionMessageType.ProofError; payload: { error: string } }
  | { type: ExtensionMessageType.RedirectBack; payload: never }
  | { type: ExtensionMessageType.TabOpened; payload: { tabId: number } }
  | {
      type: ExtensionMessageType.ProofProcessing;
      payload: {
        // as we dont have progress yet from tlsn this is optional
        progress?: number;
      };
    };

export type WebProverSessionConfig = {
  notaryUrl: string | null;
  wsProxyUrl: string | null;
  logoUrl: string | null;
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
