import {
  EXTENSION_STEP,
  type WebProofStepUserAction,
} from "../../../web-proof-commons";

export const userAction = (
  url: string,
  label: string,
  instruction: {
    text: string;
    image?: string;
  },
  assertion: {
    domElement: string;
    require: { exist: true } | { notExist: true };
  },
) => {
  return {
    url,
    label,
    instruction,
    assertion,
    step: EXTENSION_STEP.userAction,
  } as WebProofStepUserAction;
};
