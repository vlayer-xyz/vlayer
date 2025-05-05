import { useBrowsingHistory } from "./useBrowsingHistory";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { BrowsingHistoryItem } from "src/state";
import { LOADING } from "@vlayer/extension-hooks";
import { URLPattern } from "urlpattern-polyfill";
import { match, P } from "ts-pattern";
import { WebProofStep, WebProofStepNotarize } from "src/web-proof-commons";

function isNotarizeStep(step: WebProofStep): step is WebProofStepNotarize {
  return step.step === "notarize";
}

export function useProvenUrl(): BrowsingHistoryItem | null {
  const [config] = useProvingSessionConfig();
  const [browsingHistory] = useBrowsingHistory();
  const steps = match(config)
    .with(LOADING, () => [])
    .with(P.nullish, () => [])
    .with({ steps: P.array(P.any) }, ({ steps }) => {
      return steps;
    })
    .exhaustive();

  const notarizeStep = steps.find(isNotarizeStep);
  if (!notarizeStep) {
    return null;
  }

  return (
    browsingHistory.find(
      (item: BrowsingHistoryItem) =>
        (item.method as string) === notarizeStep.method &&
        new URLPattern(notarizeStep.url as string).test(item.url),
    ) ?? null
  );
}
