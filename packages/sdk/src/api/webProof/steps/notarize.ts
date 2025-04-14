import {
  EXTENSION_STEP,
  type WebProofStepNotarize,
  type RedactionConfig,
  type OutputsConfig,
} from "../../../web-proof-commons";

export const notarize = (
  url: string,
  method: string = "GET",
  label: string,
  redact?: RedactionConfig,
  outputs?: OutputsConfig,
) => {
  return {
    url,
    method,
    label,
    redact: redact ?? [],
    outputs: outputs ?? [],
    step: EXTENSION_STEP.notarize,
  } as WebProofStepNotarize;
};
