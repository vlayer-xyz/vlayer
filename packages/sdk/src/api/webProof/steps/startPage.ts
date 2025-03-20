import {
  EXTENSION_STEP,
  type WebProofStepStartPage,
} from "../../../web-proof-commons";

export const startPage = (
  url: string,
  label: string,
  autoredirect: boolean = false,
) => {
  return {
    url,
    label,
    autoredirect,
    step: EXTENSION_STEP.startPage,
  } as WebProofStepStartPage;
};
