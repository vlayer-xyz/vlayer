import type { Branded } from "../utils";
import { URLPattern } from "urlpattern-polyfill";
import { type RedactionConfig } from "./redaction";
import urlRegex from "url-regex";
import type { PresentationJSON as TLSNPresentationJSON } from "tlsn-js/src/types";
import type { OutputsConfig } from "./notarizeOutput";

export const EXTENSION_STEP = {
  expectUrl: "expectUrl",
  startPage: "startPage",
  notarize: "notarize",
  extractVariables: "extractVariables",
  clickButton: "clickButton",
} as const;

export type ExtensionStep =
  (typeof EXTENSION_STEP)[keyof typeof EXTENSION_STEP];

export const enum ExtensionAction {
  RequestWebProof = "RequestWebProof",
  NotifyZkProvingStatus = "NotifyZkProvingStatus",
  OpenSidePanel = "OpenSidePanel",
  CloseSidePanel = "CloseSidePanel",
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
    }
  | {
      action: ExtensionAction.OpenSidePanel;
    }
  | {
      action: ExtensionAction.CloseSidePanel;
    };

export enum ExtensionMessageType {
  ProofDone = "ProofDone",
  ProofError = "ProofError",
  RedirectBack = "RedirectBack",
  TabOpened = "TabOpened",
  ProofProcessing = "ProofProcessing",
  CleanProvingSessionStorageOnClose = "CleanProvingSessionStorageOnClose",
  CloseSidePanel = "CloseSidePanel",
  SidePanelClosed = "SidePanelClosed",
}

export type PresentationJSON = TLSNPresentationJSON;

export type ExtensionMessage =
  | {
      type: ExtensionMessageType.ProofDone;
      payload: {
        presentationJson: PresentationJSON;
        decodedTranscript: {
          sent: string;
          recv: string;
        };
      };
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
    }
  | { type: ExtensionMessageType.SidePanelClosed }
  | { type: ExtensionMessageType.CloseSidePanel };

export type EmptyWebProverSessionConfig = {
  notaryUrl: null;
  wsProxyUrl: null;
  logoUrl: null;
  jwtToken: null;
  steps: never[];
};

export type WebProverSessionConfig =
  | {
      notaryUrl: string;
      wsProxyUrl: string;
      logoUrl: string;
      jwtToken: string | null;
      steps: WebProofStep[];
    }
  | EmptyWebProverSessionConfig;

export function isEmptyWebProverSessionConfig(
  config: WebProverSessionConfig,
): config is EmptyWebProverSessionConfig {
  return (
    !config ||
    (!config.notaryUrl &&
      !config.wsProxyUrl &&
      !config.logoUrl &&
      config.steps.length === 0)
  );
}

export type WebProofStep =
  | WebProofStepNotarize
  | WebProofStepExpectUrl
  | WebProofStepStartPage
  | WebProofStepExtractVariables
  | WebProofStepClickButton;

export type UrlPattern = Branded<string, "UrlPattern">;

export type Url = Branded<UrlPattern, "Url">;

type BrandedStep<S extends ExtensionStep, T> = Branded<T & { step: S }, S>;

export type WebProofStepNotarize = BrandedStep<
  typeof EXTENSION_STEP.notarize,
  {
    url: UrlPattern;
    method: string;
    label: string;
    redact: RedactionConfig;
    outputs: OutputsConfig;
  }
>;

export type WebProofStepStartPage = BrandedStep<
  typeof EXTENSION_STEP.startPage,
  {
    url: Url;
    label: string;
  }
>;

export type WebProofStepExpectUrl = BrandedStep<
  typeof EXTENSION_STEP.expectUrl,
  {
    url: UrlPattern;
    label: string;
  }
>;

export type WebProofStepExtractVariables = BrandedStep<
  typeof EXTENSION_STEP.extractVariables,
  {
    label: string;
    url: UrlPattern;
    variables: Variables;
  }
>;

export type WebProofStepClickButton = BrandedStep<
  typeof EXTENSION_STEP.clickButton,
  {
    label: string;
    url: UrlPattern;
    selector: string;
  }
>;

type Header = [string, string];
export type Headers = Header[];

type Variables = Variable[];
export type Variable = {
  name: string;
  path: string;
  source: VariableSource;
};

export enum VariableSource {
  ResponseBody = "ResponseBody",
  RequestBody = "RequestBody",
  Headers = "Headers",
}

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
  const regex = urlRegex({ strict: true });
  const isUrl = regex.test(url);
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
