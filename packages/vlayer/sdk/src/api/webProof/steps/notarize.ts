import {
  EXTENSION_STEP,
  WebProofStepNotarize,
} from "../../../api/lib/types/webProofProvider";

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
