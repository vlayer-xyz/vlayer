import { BrowsingHistoryItem } from "../state/history";
import { Step, StepStatus } from "../constants";
import {
  ExtensionStep,
  UrlPattern,
  WebProofStep,
  WebProofStepUserAction,
} from "../web-proof-commons";
import { useProvingSessionConfig } from "hooks/useProvingSessionConfig.ts";
import { useBrowsingHistory } from "hooks/useBrowsingHistory.ts";
import { useZkProvingState } from "./useZkProvingState";
import { URLPattern } from "urlpattern-polyfill";
import { match, P } from "ts-pattern";
import { LOADING, useIntervalCalls } from "@vlayer/extension-hooks";
import { useNotifyOnStepCompleted } from "hooks/useNotifyOnStepCompleted.ts";
import { useCallback, useEffect, useRef, useState } from "react";
import { getActiveTabUrl, getElementOnPage } from "lib/activeTabContext.ts";

type StepCompletionCheck<T extends WebProofStep> = (
  browsingHistory: BrowsingHistoryItem[],
  step: T,
  isZkProvingDone: boolean,
) => Promise<boolean> | boolean;

type StepByType<U extends ExtensionStep> = Extract<WebProofStep, { step: U }>;

type StepCompletions = {
  [K in ExtensionStep]: StepCompletionCheck<StepByType<K>>;
};

const isUrlRequestCompleted = (
  browsingHistory: BrowsingHistoryItem[],
  step: { url: UrlPattern },
): boolean => {
  return !!browsingHistory.find((item: BrowsingHistoryItem) => {
    return new URLPattern(step.url as string).test(item.url) && item.ready;
  });
};

const wasUrlVisited = (
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

const isActiveTabUrlMatching = async (expectedUrl: string) => {
  const currentUrl = await getActiveTabUrl();
  if (!currentUrl) {
    return false;
  }
  return new URLPattern(expectedUrl).test(currentUrl);
};

const isExpectedDomElementState = async (
  _browsingHistory: BrowsingHistoryItem[],
  step: WebProofStepUserAction,
) => {
  if (!(await isActiveTabUrlMatching(step.url))) {
    return false;
  }

  let element: Element | null;
  try {
    element = await getElementOnPage(step.assertion.domElement);
  } catch (e) {
    console.error(
      `Error getting element ${step.assertion.domElement} on page:`,
      e,
    );
    return false;
  }

  if (element === null) {
    return step.assertion.require.notExist;
  }
  return step.assertion.require.exist;
};

const isStartPageStepReady = () => true;
const isStartPageStepCompleted = wasUrlVisited;

const isRedirectStepReady = () => true;
const isRedirectStepCompleted = wasUrlVisited;

const isUserActionStepReady = () => true;

const isUserActionStepCompleted = isExpectedDomElementState;

const isExpectUrlStepReady = () => true;
const isExpectUrlStepCompleted = wasUrlVisited;

const isNotarizeStepReady = isUrlRequestCompleted;
const isNotarizeStepCompleted = hasProof;

const isExtractVariablesStepReady = () => true;
const isExtractVariablesStepCompleted = () => true;

const isClickButtonStepReady = () => true;
const isClickButtonStepCompleted = () => true;

const checkStepCompletion: StepCompletions = {
  startPage: isStartPageStepCompleted,
  redirect: isRedirectStepCompleted,
  userAction: isUserActionStepCompleted,
  expectUrl: isExpectUrlStepCompleted,
  notarize: isNotarizeStepCompleted,
  extractVariables: isExtractVariablesStepCompleted,
  clickButton: isClickButtonStepCompleted,
};

function checkCompletion<T extends ExtensionStep>(
  step: T,
): StepCompletionCheck<StepByType<T>> {
  return checkStepCompletion[step];
}

const checkStepReadiness = {
  startPage: isStartPageStepReady,
  redirect: isRedirectStepReady,
  userAction: isUserActionStepReady,
  expectUrl: isExpectUrlStepReady,
  notarize: isNotarizeStepReady,
  extractVariables: isExtractVariablesStepReady,
  clickButton: isClickButtonStepReady,
};

const calculateStepStatus = async ({
  wasCompletedBefore,
  hasUncompletedStep,
  step,
  history,
  isZkProvingDone,
}: {
  wasCompletedBefore: boolean;
  hasUncompletedStep: boolean;
  step: WebProofStep;
  history: BrowsingHistoryItem[];
  isZkProvingDone: boolean;
}): Promise<StepStatus> => {
  if (wasCompletedBefore) {
    return StepStatus.Completed;
  }

  //after uncompleted step all steps can only by further no need to calculate anything
  if (hasUncompletedStep) {
    return StepStatus.Further;
  }

  // check if step is completed
  if (await checkCompletion(step.step)(history, step, isZkProvingDone)) {
    return StepStatus.Completed;
  }
  // check if step is ready
  if (checkStepReadiness[step.step](history, step)) {
    return StepStatus.Current;
  }

  return StepStatus.Further;
};

export const calculateSteps = async ({
  stepsSetup = [],
  history,
  isZkProvingDone,
  alreadyCompletedStepsCount,
}: {
  stepsSetup: WebProofStep[];
  history: BrowsingHistoryItem[];
  isZkProvingDone: boolean;
  alreadyCompletedStepsCount: number;
}) => {
  const steps: Step[] = [];
  for (let i = 0; i < stepsSetup.length; i++) {
    const currentStep = stepsSetup[i];
    const hasUncompletedStep =
      steps.length > 0 && steps.at(-1)?.status !== StepStatus.Completed;

    const status = await calculateStepStatus({
      wasCompletedBefore: i < alreadyCompletedStepsCount,
      hasUncompletedStep,
      step: currentStep,
      history,
      isZkProvingDone,
    });

    steps.push({
      step: currentStep,
      label: currentStep.label,
      link: currentStep.url,
      kind: currentStep.step,
      status,
    });
  }

  return steps;
};

export const useSteps = (): Step[] => {
  const [config] = useProvingSessionConfig();
  const [history] = useBrowsingHistory();
  const { isDone: isZkProvingDone } = useZkProvingState();
  const [steps, setSteps] = useState<Step[]>([]);
  const completedSteps = useRef(0);

  const stepsSetup = match(config)
    .with(LOADING, () => [])
    .with(P.nullish, () => [])
    .with({ steps: P.array(P.any) }, ({ steps }) => steps)
    .exhaustive();

  useEffect(() => {
    if (history.length === 0) {
      completedSteps.current = 0;
    }
  }, [history]);

  const recalculateSteps = useCallback(async () => {
    const steps = await calculateSteps({
      alreadyCompletedStepsCount: completedSteps.current,
      stepsSetup,
      history,
      isZkProvingDone,
    });
    completedSteps.current = steps.filter(
      (step) => step.status === StepStatus.Completed,
    ).length;
    setSteps(steps);
  }, [history, isZkProvingDone, stepsSetup]);

  const RECALCULATE_STEPS_TIMEOUT = 100;
  useIntervalCalls(recalculateSteps, RECALCULATE_STEPS_TIMEOUT);

  useEffect(() => void recalculateSteps(), [recalculateSteps]);

  useNotifyOnStepCompleted(stepsSetup, steps);

  return steps;
};
