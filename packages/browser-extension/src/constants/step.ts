import type {
  ExtensionStep,
  UrlPattern,
  WebProofStep,
} from "src/web-proof-commons";

export enum StepStatus {
  Completed = "completed",
  Current = "current",
  Further = "further",
}

export type Step = {
  step: WebProofStep;
  status: StepStatus;
  label: string;
  kind: ExtensionStep;
  link?: string | UrlPattern;
  buttonText?: string;
};
