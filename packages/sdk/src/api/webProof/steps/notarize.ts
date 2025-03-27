import {
  EXTENSION_STEP,
  type WebProofStepNotarize,
  type RedactionConfig,
} from "@vlayer/web-proof-commons";

export const notarize = (
  url: string,
  method: string = "GET",
  label: string,
  redact?: RedactionConfig,
) => {
  return {
    url,
    method,
    label,
    redact: redact ?? [],
    step: EXTENSION_STEP.notarize,
  } as WebProofStepNotarize;
};
