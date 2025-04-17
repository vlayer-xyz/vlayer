import { useBrowsingHistory } from "./useBrowsingHistory";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { BrowsingHistoryItem } from "../state/history";
import { LOADING } from "@vlayer/extension-hooks";
import { URLPattern } from "urlpattern-polyfill";
import { match, P } from "ts-pattern";

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

  const provenUrlAddress = steps.find(({ step }) => step === "notarize")?.url;

  return (
    browsingHistory.find((item: BrowsingHistoryItem) => {
      return provenUrlAddress
        ? new URLPattern(provenUrlAddress as string).test(item.url)
        : false;
    }) ?? null
  );
}
