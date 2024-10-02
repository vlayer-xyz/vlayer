import {
  EXTENSION_STEP,
  WebProofStepStartPage,
} from "../../../api/lib/types/webProofProvider";

export const startPage = (url: string, label: string) => {
  return {
    url,
    label,
    step: EXTENSION_STEP.startPage,
  } as WebProofStepStartPage;
};
