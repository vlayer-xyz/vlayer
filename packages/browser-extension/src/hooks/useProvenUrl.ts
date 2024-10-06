import { useBrowsingHistory } from "./useBrowsingHistory";
import { useEffect, useState } from "react";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { HistoryItem } from "../state/history";

// NOTE this will need to be refactored
// if one day we will decide to support multiple parallel proves scenario

export const useProvenUrl = () => {
  const [{ steps }] = useProvingSessionConfig();
  const [provenUrlAddress, setProvenUrlAddress] = useState("");
  const [provenUrl, setProvenUrl] = useState<HistoryItem>();
  const [browsingHistory] = useBrowsingHistory();

  useEffect(() => {
    setProvenUrlAddress(
      steps.find(({ step }) => {
        return step === "notaryUrl";
      })?.url || "",
    );
  }, [steps]);

  useEffect(() => {
    setProvenUrl(
      browsingHistory.find((item) => {
        return item.url === provenUrlAddress;
      }),
    );
  }, [browsingHistory]);

  return provenUrl;
};
