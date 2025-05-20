import type { Branded } from "../utils";
import { URLPattern } from "urlpattern-polyfill";
import { type RedactionConfig } from "./redaction";
import urlRegex from "url-regex";
import type { PresentationJSON as TLSNPresentationJSON } from "tlsn-js/src/types";
import type { OutputsConfig } from "./notarizeOutput";
import { z } from "zod";

export const EXTENSION_STEP = {
  expectUrl: "expectUrl",
  startPage: "startPage",
  redirect: "redirect",
  notarize: "notarize",
  extractVariables: "extractVariables",
  userAction: "userAction",
  clickButton: "clickButton",
} as const;

export enum ZkProvingStatus {
  NotStarted = "NotStarted",
  Proving = "Proving",
  Done = "Done",
  Error = "Error",
}

export type ExtensionStep =
  (typeof EXTENSION_STEP)[keyof typeof EXTENSION_STEP];

export enum MessageToExtensionType {
  RequestWebProof = "RequestWebProof",
  NotifyZkProvingStatus = "NotifyZkProvingStatus",
  OpenSidePanel = "OpenSidePanel",
  CloseSidePanel = "CloseSidePanel",
}

export enum LegacyMessageToExtensionType {
  Ping = "Ping",
}

export enum ExtensionInternalMessageType {
  RedirectBack = "RedirectBack",
  TabOpened = "TabOpened",
  CleanProvingSessionStorageOnClose = "CleanProvingSessionStorageOnClose",
  CloseSidePanel = "CloseSidePanel",
  ProofDone = "ProofDone",
  ProofError = "ProofError",
  ProofProcessing = "ProofProcessing",
  ResetTlsnProving = "ResetTlsnProving",
  StepCompleted = "StepCompleted",
}

export enum MessageFromExtensionType {
  RedirectBack = "RedirectBack",
  SidePanelClosed = "SidePanelClosed",
  ProofDone = "ProofDone",
  ProofError = "ProofError",
  ProofProcessing = "ProofProcessing",
  Pong = "Pong",
  StepCompleted = "StepCompleted",
}

export type LegacyMessage = {
  type: LegacyMessageToExtensionType.Ping;
};

export type MessageToExtension =
  | {
      type: MessageToExtensionType.RequestWebProof;
      payload: WebProverSessionConfig;
    }
  | {
      type: MessageToExtensionType.NotifyZkProvingStatus;
      payload: {
        status: ZkProvingStatus;
      };
    }
  | {
      type: MessageToExtensionType.OpenSidePanel;
    }
  | {
      type: MessageToExtensionType.CloseSidePanel;
    };

export type PresentationJSON = TLSNPresentationJSON;

export type ExtensionInternalMessage =
  | { type: ExtensionInternalMessageType.RedirectBack }
  | { type: ExtensionInternalMessageType.TabOpened; payload: { tabId: number } }
  | { type: ExtensionInternalMessageType.CloseSidePanel }
  | {
      type: ExtensionInternalMessageType.ProofDone;
      payload: {
        presentationJson: PresentationJSON;
        decodedTranscript: {
          sent: string;
          recv: string;
        };
      };
    }
  | {
      type: ExtensionInternalMessageType.ProofError;
      payload: { error: string };
    }
  | {
      type: ExtensionInternalMessageType.ProofProcessing;
      payload: { progress?: number };
    }
  | {
      type: ExtensionInternalMessageType.CleanProvingSessionStorageOnClose;
    }
  | {
      type: ExtensionInternalMessageType.ResetTlsnProving;
    }
  | {
      type: ExtensionInternalMessageType.StepCompleted;
      payload: {
        index: number;
        step: WebProofStep;
      };
    };

export type MessageFromExtension =
  | {
      type: MessageFromExtensionType.SidePanelClosed;
    }
  | {
      type: MessageFromExtensionType.ProofDone;
      payload: {
        presentationJson: PresentationJSON;
        decodedTranscript: {
          sent: string;
          recv: string;
        };
      };
    }
  | {
      type: MessageFromExtensionType.ProofError;
      payload: { error: string };
    }
  | {
      type: MessageFromExtensionType.ProofProcessing;
      payload: {
        progress?: number;
      };
    }
  | {
      type: MessageFromExtensionType.StepCompleted;
      payload: {
        index: number;
        step: WebProofStep;
      };
    };

export type WebProverSessionConfig = {
  notaryUrl: string;
  wsProxyUrl: string;
  logoUrl: string;
  token?: string;
  steps: WebProofStep[];
};

export type WebProofStep =
  | WebProofStepNotarize
  | WebProofStepExpectUrl
  | WebProofStepStartPage
  | WebProofStepRedirect
  | WebProofStepExtractVariables
  | WebProofStepUserAction
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

export type WebProofStepRedirect = BrandedStep<
  typeof EXTENSION_STEP.redirect,
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

export type WebProofStepUserAction = BrandedStep<
  typeof EXTENSION_STEP.userAction,
  {
    label: string;
    url: UrlPattern;
    instruction: {
      text: string;
      image?: string;
    };
    assertion: {
      domElement: string;
      require:
        | { exist: true; notExist: never }
        | { notExist: true; exist: never };
    };
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

export type Variables = Variable[];
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

const messageFromExtensionSchema = z.object({
  type: z.enum(
    Object.values<string>(MessageFromExtensionType) as [string, ...string[]],
  ),
});

const messageToExtensionSchema = z.object({
  type: z.enum(
    Object.values<string>(MessageToExtensionType) as [string, ...string[]],
  ),
});

const extensionInternalMessageSchema = z.object({
  type: z.enum(
    Object.values<string>(ExtensionInternalMessageType) as [
      string,
      ...string[],
    ],
  ),
});

const legacyPingMessageSchema = z.object({
  message: z.literal("ping"),
});

export function isMessageFromExtension(
  message: unknown,
): message is MessageFromExtension {
  return messageFromExtensionSchema.safeParse(message).success;
}

export function isLegacyPingMessage(
  message: unknown,
): message is LegacyMessage {
  return legacyPingMessageSchema.safeParse(message).success;
}

export function isMessageToExtension(
  message: unknown,
): message is MessageToExtension {
  return messageToExtensionSchema.safeParse(message).success;
}

export function isExtensionInternalMessage(
  message: unknown,
): message is ExtensionInternalMessage {
  return extensionInternalMessageSchema.safeParse(message).success;
}
