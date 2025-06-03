import { BrowsingHistoryItem } from "../state/history";
import { Step, StepStatus } from "../constants";
import {
  EXTENSION_STEP,
  UrlPattern,
  WebProofStep,
  WebProofStepUserAction,
} from "../web-proof-commons";
import { useProvingSessionConfig } from "hooks/useProvingSessionConfig";
import { useBrowsingHistory } from "hooks/useBrowsingHistory";
import { useStoredUserActionAssertions } from "hooks/useStoredUserActionAssertions";
import { useZkProvingState } from "./useZkProvingState";
import { URLPattern } from "urlpattern-polyfill";
import { match, P } from "ts-pattern";
import { LOADING, useIntervalCalls } from "@vlayer/extension-hooks";
import { useNotifyOnStepCompleted } from "hooks/useNotifyOnStepCompleted";
import { useCallback, useEffect, useMemo, useState } from "react";
import { getActiveTabUrl, getElementOnPage } from "lib/activeTabContext";

interface ActionableStep<T extends WebProofStep = WebProofStep> {
  step: T;

  isReady?(): boolean;

  isCompleted?(): boolean | Promise<boolean>;
}

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

const hasProof = (isZkProvingDone: boolean) => {
  return isZkProvingDone;
};

const isActiveTabUrlMatching = async (expectedUrl: string) => {
  const currentUrl = await getActiveTabUrl();
  if (!currentUrl) {
    return false;
  }
  return new URLPattern(expectedUrl).test(currentUrl);
};

const domStateAssertion = (
  element: Element | null,
  assertion: WebProofStepUserAction["assertion"],
) => {
  if (element === null) {
    return assertion.require.notExist;
  }
  return assertion.require.exist;
};

const isExpectedDomElementState = async (
  step: WebProofStepUserAction,
  storedAssertion: boolean | undefined,
  storeAssertion: (value: boolean) => void,
) => {
  if (!(await isActiveTabUrlMatching(step.url))) {
    return Boolean(storedAssertion);
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

  const result = domStateAssertion(element, step.assertion);
  storeAssertion(result);

  return result;
};

function intoActionableStep(
  step: WebProofStep,
  history: BrowsingHistoryItem[],
  isZkProvingDone: boolean,
  assertions: Record<string, boolean>,
  storeAssertion: (key: string, value: boolean) => void,
): ActionableStep {
  return match(step)
    .with(
      {
        step: P.union(
          EXTENSION_STEP.startPage,
          EXTENSION_STEP.redirect,
          EXTENSION_STEP.expectUrl,
        ),
      },
      (step) => ({
        step,
        isCompleted: () => wasUrlVisited(history, step),
      }),
    )
    .with({ step: EXTENSION_STEP.userAction }, (step) => ({
      step,
      isCompleted: () =>
        isExpectedDomElementState(step, assertions[step.label], (value) =>
          storeAssertion(step.label, value),
        ),
    }))
    .with({ step: EXTENSION_STEP.notarize }, (step) => ({
      step,
      isReady: () => isUrlRequestCompleted(history, step),
      isCompleted: () => hasProof(isZkProvingDone),
    }))
    .with(
      {
        step: P.union(
          EXTENSION_STEP.extractVariables,
          EXTENSION_STEP.clickButton,
        ),
      },
      (step) => ({
        step,
      }),
    )
    .exhaustive();
}

export function getActionableSteps({
  stepsSetup,
  history,
  isZkProvingDone,
  assertions,
  storeAssertion,
}: {
  stepsSetup: WebProofStep[];
  history: BrowsingHistoryItem[];
  isZkProvingDone: boolean;
  assertions: Record<string, boolean>;
  storeAssertion: (key: string, value: boolean) => void;
}) {
  return stepsSetup.map((step) =>
    intoActionableStep(
      step,
      history,
      isZkProvingDone,
      assertions,
      storeAssertion,
    ),
  );
}

const calculateStepStatus = async (
  step: ActionableStep<WebProofStep>,
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

export const calculateSteps = async (stepsSetup: ActionableStep[]) => {
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

  const actionableSteps = useMemo(
    () =>
      getActionableSteps({
        stepsSetup,
        history,
        isZkProvingDone,
        assertions,
        storeAssertion,
      }),
    [history, isZkProvingDone, stepsSetup, assertions, storeAssertion],
  );

  const recalculateSteps = useCallback(async () => {
    setSteps(await calculateSteps(actionableSteps));
  }, [actionableSteps]);

  const RECALCULATE_STEPS_TIMEOUT = 100;
  useIntervalCalls(recalculateSteps, RECALCULATE_STEPS_TIMEOUT);

  useEffect(() => void recalculateSteps(), [recalculateSteps]);

  useNotifyOnStepCompleted(stepsSetup, steps);

  return steps;
};
