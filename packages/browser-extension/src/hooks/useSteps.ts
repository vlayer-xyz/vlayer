import { BrowsingHistoryItem } from "../state/history";
import { Step, StepStatus } from "../constants";
import { UrlPattern, WebProofStep } from "../web-proof-commons";
import { useProvingSessionConfig } from "hooks/useProvingSessionConfig.ts";
import { useBrowsingHistory } from "hooks/useBrowsingHistory.ts";
import { useZkProvingState } from "./useZkProvingState";
import { URLPattern } from "urlpattern-polyfill";
import { match, P } from "ts-pattern";
import { LOADING } from "@vlayer/extension-hooks";
import { useNotifyOnStepCompleted } from "hooks/useNotifyOnStepCompleted.ts";

const isUrlRequestCompleted = (
  browsingHistory: BrowsingHistoryItem[],
  step: { url: UrlPattern },
): boolean => {
  return !!browsingHistory.find((item: BrowsingHistoryItem) => {
    return new URLPattern(step.url as string).test(item.url) && item.ready;
  });
};

const isUrlVisited = (
  browsingHistory: BrowsingHistoryItem[],
  step: { url: UrlPattern },
): boolean => {
  return !!browsingHistory.find((item: BrowsingHistoryItem) => {
    return new URLPattern(step.url as string).test(item.url);
  });
};

const hasProof = (
  _browsingHistory: BrowsingHistoryItem[],
  _step: { url: UrlPattern },
  isZkProvingDone: boolean,
) => {
  return isZkProvingDone;
};

const isStartPageStepReady = () => true;
const isStartPageStepCompleted = isUrlVisited;

const isRedirectStepReady = () => true;
const isRedirectStepCompleted = isUrlVisited;

const isUserActionStepReady = () => true;
const isUserActionStepCompleted = () => false;

const isExpectUrlStepReady = () => true;
const isExpectUrlStepCompleted = isUrlVisited;

const isNotarizeStepReady = isUrlRequestCompleted;
const isNotarizeStepCompleted = hasProof;

const isExtractVariablesStepReady = () => true;
const isExtractVariablesStepCompleted = () => true;

const isClickButtonStepReady = () => true;
const isClickButtonStepCompleted = () => true;

const checkStepCompletion = {
  startPage: isStartPageStepCompleted,
  redirect: isRedirectStepCompleted,
  userAction: isUserActionStepCompleted,
  expectUrl: isExpectUrlStepCompleted,
  notarize: isNotarizeStepCompleted,
  extractVariables: isExtractVariablesStepCompleted,
  clickButton: isClickButtonStepCompleted,
};

const checkStepReadiness = {
  startPage: isStartPageStepReady,
  redirect: isRedirectStepReady,
  userAction: isUserActionStepReady,
  expectUrl: isExpectUrlStepReady,
  notarize: isNotarizeStepReady,
  extractVariables: isExtractVariablesStepReady,
  clickButton: isClickButtonStepReady,
};

const calculateStepStatus = ({
  hasUncompletedStep,
  step,
  history,
  isZkProvingDone,
}: {
  hasUncompletedStep: boolean;
  step: WebProofStep;
  history: BrowsingHistoryItem[];
  isZkProvingDone: boolean;
}): StepStatus => {
  //after uncompleted step all steps can only by further no need to calculate anything
  if (hasUncompletedStep) {
    return StepStatus.Further;
  }
  // check if step is completed
  if (checkStepCompletion[step.step](history, step, isZkProvingDone)) {
    return StepStatus.Completed;
  }
  // check if step is ready
  if (checkStepReadiness[step.step](history, step)) {
    return StepStatus.Current;
  }

  return StepStatus.Further;
};

export const calculateSteps = ({
  stepsSetup = [],
  history,
  isZkProvingDone,
}: {
  stepsSetup: WebProofStep[];
  history: BrowsingHistoryItem[];
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
      status: calculateStepStatus({
        hasUncompletedStep,
        step: currentStep,
        history,
        isZkProvingDone,
      }),
    };
    return [...accumulator, mappedStep];
  }, [] as Step[]);
};

export const useSteps = (): Step[] => {
  const [config] = useProvingSessionConfig();
  const [history] = useBrowsingHistory();
  const { isDone: isZkProvingDone } = useZkProvingState();

  const stepsSetup = match(config)
    .with(LOADING, () => [])
    .with(P.nullish, () => [])
    .with({ steps: P.array(P.any) }, ({ steps }) => steps)
    .exhaustive();

  const steps = calculateSteps({
    stepsSetup,
    history,
    isZkProvingDone,
  });

  useNotifyOnStepCompleted(stepsSetup, steps);

  return steps;
};
