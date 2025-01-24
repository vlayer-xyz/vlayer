import {
  type WebProofRequest,
  type WebProofRequestInput,
} from "../lib/types/webProofProvider";

export const createWebProofRequest = ({
  logoUrl,
  steps,
}: WebProofRequestInput) => {
  return {
    logoUrl,
    steps,
    isWebProof: true,
  } as WebProofRequest;
};
