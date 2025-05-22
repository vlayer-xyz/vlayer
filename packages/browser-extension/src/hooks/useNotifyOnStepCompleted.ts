import {
  ExtensionInternalMessageType,
  WebProofStep,
} from "../web-proof-commons";
import { Step, StepStatus } from "src/constants";
import { useRef } from "react";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker.ts";

export const useNotifyOnStepCompleted = (
  stepsSetup: WebProofStep[],
  currentStepsProgress: Step[],
) => {
  const completedSteps = useRef(0);

  const completedStepsCount = currentStepsProgress.filter(
    (step) => step.status === StepStatus.Completed,
  ).length;

  if (completedStepsCount > completedSteps.current) {
    for (let i = completedSteps.current; i < completedStepsCount; i++) {
      void sendMessageToServiceWorker({
        type: ExtensionInternalMessageType.StepCompleted,
        payload: {
          index: i,
          step: stepsSetup[i],
        },
      });
    }
    completedSteps.current = completedStepsCount;
  }
};
