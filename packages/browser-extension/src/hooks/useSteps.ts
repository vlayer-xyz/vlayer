import { HistoryItem } from "../state/history";
import { Step, StepStatus } from "../constants";
import { WebProofStep } from "../web-proof-commons";
import { useProvingSessionConfig } from "hooks/useProvingSessionConfig.ts";
import { useBrowsingHistory } from "hooks/useBrowsingHistory.ts";
import { useZkProvingState } from "./useZkProvingState";

const isStartPageStepCompleted = (
  browsingHistory: HistoryItem[],
  step: { url: string },
): boolean => {
  // REFACTOR:  i would rename top level history to browsing to avoid history.history
  return !!browsingHistory.find((item: HistoryItem) => {
    return item.url.startsWith(step.url) && item.ready;
  });
};

const isStartPageStepReady = () => true;

const isExpectUrlStepCompleted = (
  browsingHistory: HistoryItem[],
  step: { url: string },
): boolean => {
  return !!browsingHistory.find((item: HistoryItem) => {
    return item.url.startsWith(step.url) && item.ready;
  });
};

const isExpectUrlStepReady = () => true;

const isNotarizeStepReady = (
  browsingHistory: HistoryItem[],
  step: { url: string },
): boolean => {
  return !!browsingHistory.find((item: HistoryItem) => {
    return item.url.startsWith(step.url) && item.ready;
  });
};

const isNotarizeStepCompleted = (
  _browsingHistory: HistoryItem[],
  _step: { url: string },
  hasProof: boolean,
) => {
  return hasProof;
};

const checkStepCompletion = {
  startPage: isStartPageStepCompleted,
  expectUrl: isExpectUrlStepCompleted,
  notarize: isNotarizeStepCompleted,
};

const checkStepReadiness = {
  startPage: isStartPageStepReady,
  expectUrl: isExpectUrlStepReady,
  notarize: isNotarizeStepReady,
};

export const calculateSteps = ({
  stepsSetup = [],
  history,
  isZkProvingDone,
}: {
  stepsSetup: WebProofStep[];
  history: HistoryItem[];
  isZkProvingDone: boolean;
}) => {
  return stepsSetup.reduce((accumulator, currentStep) => {
    const hasUncompletedStep =
      accumulator.length > 0 &&
      accumulator[accumulator.length - 1]?.status !== StepStatus.Completed;
    const mappedStep = {
      label: currentStep.label,
      link: currentStep.url,
      kind: currentStep.step,
      // all steps after first uncompleted are further
      status: hasUncompletedStep
        ? StepStatus.Further
        : checkStepCompletion[currentStep.step](
              history,
              currentStep,
              isZkProvingDone,
            )
          ? StepStatus.Completed
          : checkStepReadiness[currentStep.step](history, currentStep)
            ? StepStatus.Current
            : StepStatus.Further,
    };
    return [...accumulator, mappedStep];
  }, [] as Step[]);
};

export const useSteps = (): Step[] => {
  const [config] = useProvingSessionConfig();
  const [history] = useBrowsingHistory();
  const { isDone: isZkProvingDone } = useZkProvingState();
  return calculateSteps({
    stepsSetup: config.steps,
    history,
    isZkProvingDone,
  });
};
