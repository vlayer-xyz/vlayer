import {
  type Variables,
  EXTENSION_STEP,
  type WebProofStepExtractVariables,
} from "src/web-proof-commons";

export const extractVariables = (
  url: string,
  label: string,
  variables: Variables,
) => {
  return {
    url,
    label,
    variables,
    step: EXTENSION_STEP.extractVariables,
  } as WebProofStepExtractVariables;
};
