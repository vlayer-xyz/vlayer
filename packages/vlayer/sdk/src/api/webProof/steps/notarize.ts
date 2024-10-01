import { WebProofStepNotarize } from "types/webProofProvider.ts";

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
