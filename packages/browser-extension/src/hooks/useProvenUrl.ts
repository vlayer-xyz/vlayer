import { useBrowsingHistory } from "./useBrowsingHistory";
import { useMemo } from "react";
import { useProvingSessionConfig } from "./useProvingSessionConfig";

// NOTE this will need to be refactored
// if one day we will decide to support multiple parallel proves scenario

export const useProvenUrl = () => {
  const [{ steps }] = useProvingSessionConfig();
  const [browsingHistory] = useBrowsingHistory();
  return useMemo(() => {
    const provenUrlAddress =
      steps?.find(({ step }) => step === "notarize")?.url || "";
    return browsingHistory.find((item) => item.url.includes(provenUrlAddress));
  }, [steps, browsingHistory]);
};
