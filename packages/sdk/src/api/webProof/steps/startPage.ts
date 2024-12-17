import {
  EXTENSION_STEP,
  type WebProofStepStartPage,
} from "../../../web-proof-commons";

export const startPage = (url: string, label: string) => {
  return {
    url,
    label,
    step: EXTENSION_STEP.startPage,
  } as WebProofStepStartPage;
};
