import { useEffect, useState } from "react";
import { type Abi, type ContractFunctionName } from "viem";
import { useProofContext } from "../context";
import { WebProofRequestStatus } from "../types";
import { MessageFromExtensionType, type WebProofConfig } from "@vlayer/sdk";

export const useWebProof = (
  webProofRequest: WebProofConfig<Abi, ContractFunctionName>,
) => {
  const { webProofProvider } = useProofContext();
  const [webProof, setWebProof] = useState<{
    webProofJson: string;
  } | null>(null);
  const [error, setError] = useState<Error | null>(null);
  const [status, setStatus] = useState<WebProofRequestStatus>(
    WebProofRequestStatus.idle,
  );

  useEffect(() => {
    webProofProvider.addEventListeners(
      MessageFromExtensionType.ProofDone,
      ({ payload: { presentationJson } }) => {
        setWebProof({
          webProofJson: JSON.stringify({ presentationJson: presentationJson }),
        });
        setStatus(WebProofRequestStatus.success);
      },
    );

    webProofProvider.addEventListeners(
      MessageFromExtensionType.ProofError,
      ({ payload: { error } }) => {
        setError(new Error(error));
        setStatus(WebProofRequestStatus.error);
      },
    );
  }, []);

  return {
    webProof,
    error,
    status,
    isIdle: status === WebProofRequestStatus.idle,
    isPending: status === WebProofRequestStatus.pending,
    isError: status === WebProofRequestStatus.error,
    isSuccess: status === WebProofRequestStatus.success,
    requestWebProof: () => {
      setStatus(WebProofRequestStatus.pending);
      webProofProvider.requestWebProof(webProofRequest);
    },
  };
};
