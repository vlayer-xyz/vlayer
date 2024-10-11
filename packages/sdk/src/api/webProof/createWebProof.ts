import { WebProofSetup, WebProofSetupInput } from "types/webProofProvider.ts";

export const createWebProof = ({ logoUrl, steps }: WebProofSetupInput) => {
  return {
    logoUrl,
    steps,
    isWebProof: true,
  } as WebProofSetup;
};
