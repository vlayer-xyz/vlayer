import {
  StepKind,
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
    kind: StepKind.notarize,
  } as WebProofStepNotarize;
};
