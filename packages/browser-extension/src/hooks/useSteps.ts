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
import { LOADING } from "@vlayer/extension-hooks";
import { useNotifyOnStepCompleted } from "hooks/useNotifyOnStepCompleted.ts";
import { useEffect, useState } from "react";
import { getElementOnPage } from "lib/scripting.ts";

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

const isUserStepCompleted = async (
  _browsingHistory: BrowsingHistoryItem[],
  step: WebProofStepUserAction,
) => {
  const element = await getElementOnPage(step.action.selector);
  if (typeof step.action.expected === "boolean") {
    return step.action.expected === !!element;
  }
  return element === step.action.expected;
};

const isStartPageStepReady = () => true;
const isStartPageStepCompleted = isUrlVisited;

const isRedirectStepReady = () => true;
const isRedirectStepCompleted = isUrlVisited;

const isUserActionStepReady = () => true;

const isUserActionStepCompleted = isUserStepCompleted;

const isExpectUrlStepReady = () => true;
const isExpectUrlStepCompleted = isUrlVisited;

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
  hasUncompletedStep,
  step,
  history,
  isZkProvingDone,
}: {
  hasUncompletedStep: boolean;
  step: WebProofStep;
  history: BrowsingHistoryItem[];
  isZkProvingDone: boolean;
}): Promise<StepStatus> => {
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
}: {
  stepsSetup: WebProofStep[];
  history: BrowsingHistoryItem[];
  isZkProvingDone: boolean;
}) => {
  const steps: Step[] = [];

  for (const currentStep of stepsSetup) {
    const hasUncompletedStep =
      steps.length > 0 && steps.at(-1)?.status !== StepStatus.Completed;

    steps.push({
      step: currentStep,
      label: currentStep.label,
      link: currentStep.url,
      kind: currentStep.step,
      status: await calculateStepStatus({
        hasUncompletedStep,
        step: currentStep,
        history,
        isZkProvingDone,
      }),
    });
  }

  return steps;
};

export const useSteps = (): Step[] => {
  const [config] = useProvingSessionConfig();
  const [history] = useBrowsingHistory();
  const { isDone: isZkProvingDone } = useZkProvingState();
  const [steps, setSteps] = useState<Step[]>([]);

  const stepsSetup = match(config)
    .with(LOADING, () => [])
    .with(P.nullish, () => [])
    .with({ steps: P.array(P.any) }, ({ steps }) => steps)
    .exhaustive();

  useEffect(() => {
    calculateSteps({
      stepsSetup,
      history,
      isZkProvingDone,
    }).then(setSteps);
  }, [stepsSetup, history, isZkProvingDone]);

  useNotifyOnStepCompleted(stepsSetup, steps);

  return steps;
};
