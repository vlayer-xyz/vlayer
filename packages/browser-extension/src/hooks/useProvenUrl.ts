import { useBrowsingHistory } from "./useBrowsingHistory";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { HistoryItem } from "../state/history";
import { useTrackHistory } from "./useTrackHistory";

export function useProvenUrl(): HistoryItem | null {
  useTrackHistory();
  const [config] = useProvingSessionConfig();
  const [browsingHistory] = useBrowsingHistory();
  const steps = config?.steps || [];
  const provenUrlAddress = steps.find(({ step }) => step === "notarize")?.url;

  return (
    browsingHistory.find((item) =>
      item.url.includes(provenUrlAddress as string),
    ) ?? null
  );
}
