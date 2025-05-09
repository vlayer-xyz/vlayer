import {
  EXTENSION_STEP,
  type WebProofStepRedirect,
} from "../../../web-proof-commons";

export const redirect = (url: string, label: string) => {
  return {
    url,
    label,
    step: EXTENSION_STEP.redirect,
  } as WebProofStepRedirect;
};
