import {
  EXTENSION_STEP,
  type WebProofStepUserAction,
} from "../../../web-proof-commons";

export const userAction = (
  url: string,
  label: string,
  text: string,
  action: {
    selector: string;
    expected: string | boolean;
  },
  image?: string,
) => {
  return {
    url,
    label,
    text,
    action,
    image,
    step: EXTENSION_STEP.userAction,
  } as WebProofStepUserAction;
};
