export enum StepStatus {
  Completed = "completed",
  Current = "current",
  Further = "further",
}

export type Step = {
  status: StepStatus;
  label: string;
  kind: "expectUrl" | "notarize" | "startPage";
  link?: string;
  buttonText?: string;
};
