import {
  EXTENSION_STEP,
  WebProofStepNotarize,
} from "../../../web-proof-commons";

export const notarize = (
  url: string,
  method: string = "GET",
  label: string,
) => {
  return {
    url,
    method,
    label,
    step: EXTENSION_STEP.notarize,
  } as WebProofStepNotarize;
};
