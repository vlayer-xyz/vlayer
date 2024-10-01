
import { StepKind, WebProofStepExpectUrl } from "../../../api/lib/types/webProofProvider";

export const expectUrl = (url: string, label: string) => {
  return {
    url,
    label,
    kind: StepKind.expectUrl,
  } as WebProofStepExpectUrl;
};
