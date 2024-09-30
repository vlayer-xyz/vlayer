import { Branded } from "types/utils.ts";

export type WebProofStepStartPage = Branded<
  {
    url: string;
    label: string;
  },
  "startPage"
>;
export const startPage = (url: string, label: string) => {
  return {
    url,
    label,
  } as WebProofStepStartPage;
};
