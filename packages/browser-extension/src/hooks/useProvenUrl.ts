import { useBrowsingHistory } from "./useBrowsingHistory";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { BrowsingHistoryItem } from "../state/history";
import { LOADING } from "@vlayer/extension-hooks";
import { URLPattern } from "urlpattern-polyfill";

export function useProvenUrl(): BrowsingHistoryItem | null {
  const [config] = useProvingSessionConfig();
  const [browsingHistory] = useBrowsingHistory();

  const steps = config !== LOADING ? config?.steps || [] : [];
  const provenUrlAddress = steps.find(({ step }) => step === "notarize")?.url;
  return (
    browsingHistory.find((item: BrowsingHistoryItem) => {
      return new URLPattern(provenUrlAddress as string).test(item.url);
    }) ?? null
  );
}
