import { useEffect, useState } from "react";
import { type Abi, type ContractFunctionName } from "viem";
import { useProofContext } from "../context";
import { WebProofRequestStatus } from "../types";
import { ExtensionMessageType, type GetWebProofArgs } from "@vlayer/sdk";

export const useWebProof = (
  webProofRequest: GetWebProofArgs<Abi, ContractFunctionName>,
) => {
  const { webProofProvider } = useProofContext();
  const [webProof, setWebProof] = useState<{
    webProofJson: string;
  } | null>(null);
  const [decodedTranscript, setDecodedTranscript] = useState<{
    sent: string;
    recv: string;
  } | null>(null);
  const [error, setError] = useState<Error | null>(null);
  const [status, setStatus] = useState<WebProofRequestStatus>(
    WebProofRequestStatus.idle,
  );

  useEffect(() => {
    webProofProvider.addEventListeners(
      ExtensionMessageType.ProofDone,
      ({ payload: { presentationJSON, decodedTranscript } }) => {
        setWebProof({
          webProofJson: JSON.stringify(presentationJSON),
        });
        setDecodedTranscript(decodedTranscript);
        setStatus(WebProofRequestStatus.success);
      },
    );

    webProofProvider.addEventListeners(
      ExtensionMessageType.ProofError,
      ({ payload: { error } }) => {
        setError(new Error(error));
        setStatus(WebProofRequestStatus.error);
      },
    );
  }, []);

  return {
    webProof,
    decodedTranscript,
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
