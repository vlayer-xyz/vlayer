import { useBrowsingHistory } from "./useBrowsingHistory";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { HistoryItem } from "../state/history";

export function useProvenUrl(): HistoryItem | undefined {
  const [config] = useProvingSessionConfig();
  const [browsingHistory] = useBrowsingHistory();

  const steps = config?.steps || [];
  const provenUrlAddress = steps.find(({ step }) => step === "notarize")?.url;

  if (!provenUrlAddress) {
    return undefined;
  }

  return browsingHistory.find((item) => item.url.includes(provenUrlAddress));
}
