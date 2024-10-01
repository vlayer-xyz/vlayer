import {
  StepKind,
  WebProofStepStartPage,
} from "../../../api/lib/types/webProofProvider";

export const startPage = (url: string, label: string) => {
  return {
    url,
    label,
    kind: StepKind.startPage,
  } as WebProofStepStartPage;
};
