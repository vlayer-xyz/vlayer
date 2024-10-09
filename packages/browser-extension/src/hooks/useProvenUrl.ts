import { useBrowsingHistory } from "./useBrowsingHistory";
import { useMemo } from "react";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { HistoryItem } from "../state/history";

// NOTE this will need to be refactored
// if one day we will decide to support multiple parallel proves scenario

export function useProvenUrl(): HistoryItem | undefined {
  const [config] = useProvingSessionConfig();
  const steps = config?.steps || [];
  const [browsingHistory] = useBrowsingHistory();
  return useMemo(() => {
    const provenUrlAddress = steps.find(({ step }) => step === "notarize")?.url;
    if (provenUrlAddress) {
      return browsingHistory.find((item) =>
        item.url.includes(provenUrlAddress),
      );
    } else {
      return undefined;
    }
  }, [steps, browsingHistory]);
}
