import { Branded } from "types/utils";

export type WebProofStepExpectUrl = Branded<
  {
    url: string;
    label: string;
  },
  "expectUrl"
>;

export const expectUrl = (url: string, label: string) => {
  return {
    url,
    label,
  } as WebProofStepExpectUrl;
};

const x = expectUrl("d", "b");
