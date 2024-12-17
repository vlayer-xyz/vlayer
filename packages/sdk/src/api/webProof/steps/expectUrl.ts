import {
  EXTENSION_STEP,
  type WebProofStepExpectUrl,
} from "../../../web-proof-commons";

export const expectUrl = (url: string, label: string) => {
  return {
    url,
    label,
    step: EXTENSION_STEP.expectUrl,
  } as WebProofStepExpectUrl;
};
