import { HistoryItem } from "../state/history";
import { Step, StepStatus } from "../constants";
import { useTlsnProver } from "hooks/useTlsnProver";
import { WebProofStep } from "../web-proof-commons";
import { useProvingSessionConfig } from "hooks/useProvingSessionConfig.ts";
import { useBrowsingHistory } from "hooks/useBrowsingHistory.ts";
import { match } from "path-to-regexp";
import urlPattern from "url-pattern";
import { s } from "framer-motion/client";

window.match = match;
window.urlPattern = urlPattern;
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

const isNotarizeGqlStepReady = (
  browsingHistory: HistoryItem[],
  step: { url: string; query: Record<string, string> },
): boolean => {
  console.log("checking grapql ready", browsingHistory);
  return !!browsingHistory.find((item: HistoryItem) => {
    try {
      console.log("item");
      const stepUrl = new URL(step.url);
      const itemUrl = new URL(item.url);
      const pattern = new urlPattern(stepUrl.pathname);

      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
      const matchesQuery = pattern.match(itemUrl.pathname);

      if (stepUrl.origin === itemUrl.origin && matchesQuery) {
        console.log("matchesQuery", step);
        const gqlParams = itemUrl.searchParams;
        //@ts-expect-error ddd
        const gqlQueryMatch = Object.keys(step.query).every((query) => {
          //@ts-expect-error derwer
          const actualValue = JSON.parse(
            decodeURIComponent(gqlParams.get(query)),
          );
          console.log("actualValue", actualValue);
          console.log("step.query[query]", step.query[query]);
          return (
            JSON.stringify(actualValue) === JSON.stringify(step.query[query])
          );
        });
        console.log("gqlQueryMatch", gqlQueryMatch);

        return true;
      }
    } catch (e) {
      console.log("error", e);
    }

    return item.url.startsWith(step.url) && item.ready;
  });
};

const isNotarizeGqlStepCompleted = (
  _browsingHistory: HistoryItem[],
  _step: { url: string },
  hasProof: boolean,
) => {
  console.log("checking grapql complete", hasProof);
  // return hasProof;
  return false;
};

const checkStepCompletion = {
  startPage: isStartPageStepCompleted,
  expectUrl: isExpectUrlStepCompleted,
  notarize: isNotarizeStepCompleted,
  notarizeGql: isNotarizeGqlStepCompleted,
};

const checkStepReadiness = {
  startPage: isStartPageStepReady,
  expectUrl: isExpectUrlStepReady,
  notarize: isNotarizeStepReady,
  notarizeGql: isNotarizeGqlStepReady,
};

export const calculateSteps = ({
  stepsSetup = [],
  proof,
  history,
}: {
  stepsSetup: WebProofStep[];
  history: HistoryItem[];
  proof: object | null;
}) => {
  return stepsSetup.reduce((accumulator, currentStep) => {
    // console.log("currentStep", currentStep);
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

export const useSteps = (): Step[] => {
  const [config] = useProvingSessionConfig();
  const [history] = useBrowsingHistory();
  const { proof } = useTlsnProver();

  return calculateSteps({ stepsSetup: config.steps, history, proof });
};
