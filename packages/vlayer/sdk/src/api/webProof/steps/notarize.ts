import { Branded } from "types/utils";

export type WebProofStepNotarize = Branded<
  {
    url: string;
    method: string;
    label: string;
  },
  "notarize"
>;
export const notarize = (
  url: string,
  method: string = "GET",
  label: string,
) => {
  return {
    url,
    method,
    label,
  } as WebProofStepNotarize;
};
