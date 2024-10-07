// // this is placeholder implementation

import { useLocalStorage } from "@vlayer/extension-hooks";
import { HistoryItem } from "../state/history";
import { Step, StepStatus } from "../constants";
import { useTlsnProver } from "hooks/useTlsnProver";

// NOTE : here we should use proper types imported from commons once those are ready

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
  // REFACTOR:  i would rename top level history to browsing to avoid history.history
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

export const useSteps = (): Step[] => {
  // get steps config
  const [{ steps }] = useLocalStorage<{
    steps: {
      url: string;
      label: string;
      step: "expectUrl" | "notarize" | "startPage";
    }[];
  }>("webProverSessionConfig", { steps: [] });

  //read browsing history
  const [history] = useLocalStorage<HistoryItem[]>("history", []);

  //get tlsn proof
  const { proof } = useTlsnProver();

  return steps.reduce((accumulator, currentStep) => {
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
        : checkStepCompletion[currentStep.step](history, currentStep, !!proof)
          ? StepStatus.Completed
          : checkStepReadiness[currentStep.step](history, currentStep)
            ? StepStatus.Current
            : StepStatus.Further,
    };
    return [...accumulator, mappedStep];
  }, [] as Step[]);
};
