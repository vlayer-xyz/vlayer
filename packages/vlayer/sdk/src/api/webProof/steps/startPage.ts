import { WebProofStepStartPage } from "types/webProofProvider.ts";

export const startPage = (url: string, label: string) => {
  return {
    url,
    label,
  } as WebProofStepStartPage;
};
