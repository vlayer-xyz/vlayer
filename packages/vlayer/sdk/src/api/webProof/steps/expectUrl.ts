import {
  EXTENSION_STEP,
  WebProofStepExpectUrl,
} from "../../../api/lib/types/webProofProvider";

export const expectUrl = (url: string, label: string) => {
  return {
    url,
    label,
    step: EXTENSION_STEP.expectUrl,
  } as WebProofStepExpectUrl;
};
