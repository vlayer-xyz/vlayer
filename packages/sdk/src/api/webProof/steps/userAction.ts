import {
  EXTENSION_STEP,
  type WebProofStepUserAction,
} from "../../../web-proof-commons";

export const userAction = (
  url: string,
  label: string,
  text: string,
  image?: string,
) => {
  return {
    url,
    label,
    text,
    image,
    step: EXTENSION_STEP.userAction,
  } as WebProofStepUserAction;
};
