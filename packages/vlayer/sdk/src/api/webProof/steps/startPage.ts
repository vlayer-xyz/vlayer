import { StepKind, WebProofStepStartPage } from "types/webProofProvider.ts";

export const startPage = (url: string, label: string) => {
  return {
    url,
    label,
    kind: StepKind.startPage,
  } as WebProofStepStartPage;
};
