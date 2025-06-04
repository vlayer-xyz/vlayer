import { URLPattern } from "urlpattern-polyfill";
import { match, P } from "ts-pattern";
import {
  EXTENSION_STEP,
  type UrlPattern,
  type WebProofStep,
} from "../../web-proof-commons";
import type { BrowsingHistoryItem } from "src/state";
import { isExpectedDomElementState } from "./domStateAssertions";

export interface InteractiveStep<T extends WebProofStep = WebProofStep> {
  step: T;

  isReady?(): boolean;

  isCompleted?(): boolean | Promise<boolean>;
}

export type InteractiveStepsConfig = {
  history: BrowsingHistoryItem[];
  isZkProvingDone: boolean;
  assertions: Record<string, boolean>;
  storeAssertion: (key: string, value: boolean) => void;
};

export const intoInteractiveStep = (
  step: WebProofStep,
  {
    history,
    isZkProvingDone,
    assertions,
    storeAssertion,
  }: InteractiveStepsConfig,
): InteractiveStep =>
  match(step)
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

export const getInteractiveSteps = (
  stepsSetup: WebProofStep[],
  config: InteractiveStepsConfig,
) => stepsSetup.map((step) => intoInteractiveStep(step, config));

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
