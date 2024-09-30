import type {
  WebProofStepExpectUrl,
  WebProofStepNotarize,
  WebProofStepStartPage,
} from "./steps";
import { Branded } from "types/utils.ts";

type WebProofSetupInput = {
  logoUrl: string;
  steps: [WebProofStepExpectUrl, WebProofStepStartPage, WebProofStepStartPage];
};

export type WebProofSetup = Branded<
  WebProofSetupInput & {
    isWebProof: true;
  },
  "webProof"
>;

export const createWebProof = ({ logoUrl, steps }: WebProofSetupInput) => {
  return {
    logoUrl,
    steps,
    isWebProof: true,
  } as WebProofSetup;
};
