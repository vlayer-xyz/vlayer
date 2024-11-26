import { UrlPattern } from "src/web-proof-commons";

export enum StepStatus {
  Completed = "completed",
  Current = "current",
  Further = "further",
}

export type Step = {
  status: StepStatus;
  label: string;
  kind: "expectUrl" | "notarize" | "startPage";
  link?: string | UrlPattern;
  buttonText?: string;
};
