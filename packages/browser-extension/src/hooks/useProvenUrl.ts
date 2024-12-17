import { useBrowsingHistory } from "./useBrowsingHistory";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { BrowsingHistoryItem } from "../state/history";
import { LOADING } from "@vlayer/extension-hooks";

export function useProvenUrl(): BrowsingHistoryItem | null {
  const [config] = useProvingSessionConfig();
  const [browsingHistory] = useBrowsingHistory();

  const steps = config !== LOADING ? config.steps : [];
  const provenUrlAddress = steps.find(({ step }) => step === "notarize")?.url;

  return (
    browsingHistory.find((item) =>
      item.url.includes(provenUrlAddress as string),
    ) ?? null
  );
}
