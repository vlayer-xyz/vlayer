import { EXTENSION_STEP, WebProofStepExpectUrl } from "@vlayer/web-proof-commons";

export const expectUrl = (url: string, label: string) => {
  return {
    url,
    label,
    step: EXTENSION_STEP.expectUrl,
  } as WebProofStepExpectUrl;
};
