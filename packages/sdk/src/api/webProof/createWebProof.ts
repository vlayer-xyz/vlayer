import { WebProofSetup, WebProofSetupInput } from "types/webProofProvider";

export const createWebProof = ({ logoUrl, steps }: WebProofSetupInput) => {
  return {
    logoUrl,
    steps,
    isWebProof: true,
  } as WebProofSetup;
};
