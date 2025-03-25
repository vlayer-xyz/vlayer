import type { ExtensionStep, UrlPattern } from "src/web-proof-commons";

export enum StepStatus {
  Completed = "completed",
  Current = "current",
  Further = "further",
}

export type Step = {
  status: StepStatus;
  label: string;
  kind: ExtensionStep;
  link?: string | UrlPattern;
  buttonText?: string;
};
