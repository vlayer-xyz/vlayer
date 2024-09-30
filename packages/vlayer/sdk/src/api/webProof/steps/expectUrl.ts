import { Branded } from "types/utils";

import { WebProofStepExpectUrl } from "types/webProofProvider.ts";

export const expectUrl = (url: string, label: string) => {
  return {
    url,
    label,
  } as WebProofStepExpectUrl;
};
