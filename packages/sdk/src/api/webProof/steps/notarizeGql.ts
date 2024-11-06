import {
  EXTENSION_STEP,
  WebProofStepNotarizeGql,
} from "../../../web-proof-commons";

export const notarizeGql = (
  url: string,
  query: object,
  label: string,
  method: string = "GET",
) => {
  return {
    url,
    label,
    method,
    query,
    step: EXTENSION_STEP.notarizeGql,
  } as WebProofStepNotarizeGql;
};
