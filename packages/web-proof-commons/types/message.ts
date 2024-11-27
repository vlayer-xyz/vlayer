import type { Branded } from "../utils";
import { URLPattern } from "urlpattern-polyfill";
import urlRegex from "url-regex";
import type { PresentationJSON as TLSNPresentationJSON } from "tlsn-js/src/types";

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

export type PresentationJSON = TLSNPresentationJSON;

export type ExtensionMessage =
  | {
      type: ExtensionMessageType.ProofDone;
      payload: { proof: PresentationJSON };
    }
  | { type: ExtensionMessageType.ProofError; payload: { error: string } }
  | { type: ExtensionMessageType.RedirectBack }
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

export type UrlPattern = Branded<string, "UrlPattern">;

export type Url = Branded<UrlPattern, "Url">;

export type WebProofStepNotarize = Branded<
  {
    url: UrlPattern;
    method: string;
    label: string;
    step: typeof EXTENSION_STEP.notarize;
  },
  "notarize"
>;

export type WebProofStepStartPage = Branded<
  {
    url: Url;
    label: string;
    step: typeof EXTENSION_STEP.startPage;
  },
  "startPage"
>;

export type WebProofStepExpectUrl = Branded<
  {
    url: UrlPattern;
    label: string;
    step: typeof EXTENSION_STEP.expectUrl;
  },
  "expectUrl"
>;

export enum StepValidationErrors {
  InvalidUrl = "InvalidUrl",
  InvalidUrlPattern = "InvalidUrlPattern",
}

export enum StepValidationErrorMessage {
  InvalidUrl = "Wrong url",
  InvalidUrlPattern = "Wrong url pattern",
}

export class StepValidationError extends Error {
  constructor(message: string, name: StepValidationErrors) {
    super(message);
    this.name = name;
  }
}

export function assertUrl(url: string): asserts url is Url {
  const isUrl = urlRegex({ strict: true }).test(url);
  if (!isUrl) {
    throw new StepValidationError(
      `${StepValidationErrorMessage.InvalidUrl}: ${url}`,
      StepValidationErrors.InvalidUrl,
    );
  }
}

export function assertUrlPattern(
  urlPattern: string,
): asserts urlPattern is UrlPattern {
  try {
    new URLPattern(urlPattern);
  } catch {
    throw new StepValidationError(
      `${StepValidationErrorMessage.InvalidUrlPattern}: ${urlPattern} `,
      StepValidationErrors.InvalidUrlPattern,
    );
  }
}
