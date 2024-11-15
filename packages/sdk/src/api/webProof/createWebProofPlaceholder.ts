import { WebProofSetup, WebProofSetupInput } from "types/webProofProvider";

export const createWebProofPlaceholder = ({ logoUrl, steps }: WebProofSetupInput) => {
  return {
    logoUrl,
    steps,
    isWebProof: true,
  } as WebProofSetup;
};
