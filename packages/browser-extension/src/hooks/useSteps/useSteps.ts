import { useCallback, useEffect, useMemo, useState } from "react";
import { match, P } from "ts-pattern";
import { LOADING, useIntervalCalls } from "@vlayer/extension-hooks";
import { type Step, StepStatus } from "src/constants";
import { useProvingSessionConfig } from "hooks/useProvingSessionConfig";
import { useBrowsingHistory } from "hooks/useBrowsingHistory";
import { useStoredUserActionAssertions } from "hooks/useStoredUserActionAssertions";
import { useNotifyOnStepCompleted } from "hooks/useNotifyOnStepCompleted";
import { useZkProvingState } from "../useZkProvingState";
import { getInteractiveSteps, type InteractiveStep } from "./interactiveSteps";

const calculateStepStatus = async (
  step: InteractiveStep,
  hasUncompletedStep: boolean,
): Promise<StepStatus> => {
  //after uncompleted step all steps can only by further no need to calculate anything
  if (hasUncompletedStep) {
    return StepStatus.Further;
  }

  // check if step is completed
  if (step.isCompleted === undefined || (await step.isCompleted())) {
    return StepStatus.Completed;
  }
  // check if step is ready
  if (step.isReady === undefined || step.isReady()) {
    return StepStatus.Current;
  }

  return StepStatus.Further;
};

export const calculateSteps = async (stepsSetup: InteractiveStep[]) => {
  const steps: Step[] = [];

  for (const currentStep of stepsSetup) {
    const hasUncompletedStep =
      steps.length > 0 && steps.at(-1)?.status !== StepStatus.Completed;

    steps.push({
      step: currentStep.step,
      label: currentStep.step.label,
      link: currentStep.step.url,
      kind: currentStep.step.step,
      status: await calculateStepStatus(currentStep, hasUncompletedStep),
    });
  }

  return steps;
};

export const useSteps = (): Step[] => {
  const [config] = useProvingSessionConfig();
  const [history] = useBrowsingHistory();
  const [assertions, storeAssertion] = useStoredUserActionAssertions();
  const { isDone: isZkProvingDone } = useZkProvingState();
  const [steps, setSteps] = useState<Step[]>([]);

  const stepsSetup = match(config)
    .with(LOADING, () => [])
    .with(P.nullish, () => [])
    .with({ steps: P.array(P.any) }, ({ steps }) => steps)
    .exhaustive();

  const interactiveSteps = useMemo(
    () =>
      getInteractiveSteps(stepsSetup, {
        history,
        isZkProvingDone,
        assertions,
        storeAssertion,
      }),
    [history, isZkProvingDone, stepsSetup, assertions, storeAssertion],
  );

  const recalculateSteps = useCallback(async () => {
    setSteps(await calculateSteps(interactiveSteps));
  }, [interactiveSteps]);

  const RECALCULATE_STEPS_TIMEOUT = 500;
  useIntervalCalls(recalculateSteps, RECALCULATE_STEPS_TIMEOUT);

  useEffect(() => void recalculateSteps(), [recalculateSteps]);

  useNotifyOnStepCompleted(stepsSetup, steps);

  return steps;
};
