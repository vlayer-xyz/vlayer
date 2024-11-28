import {
  EXTENSION_STEP,
  WebProofStepNotarize,
} from "../../../web-proof-commons";

export const notarize = (
  url: string,
  method: string = "GET",
  label: string,
  jsonRevealPath: string | undefined = undefined,
) => {
  return {
    url,
    method,
    label,
    jsonRevealPath,
    step: EXTENSION_STEP.notarize,
  } as WebProofStepNotarize;
};
