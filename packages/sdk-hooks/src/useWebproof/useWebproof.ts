import { useEffect, useState } from "react";
import { type Abi, type ContractFunctionName } from "viem";
import { useVlayerContext } from "../context";
import { WebProofRequestStatus } from "../types";
import {
  ExtensionMessageType,
  type GetWebProofArgs,
  type PresentationJSON,
} from "@vlayer/sdk";

export const useWebproof = (
  webProofRequest: GetWebProofArgs<Abi, ContractFunctionName>,
) => {
  const { webProofProvider } = useVlayerContext();
  const [webProof, setWebProof] = useState<PresentationJSON | null>(null);
  const [error, setError] = useState<Error | null>(null);
  const [status, setStatus] = useState<WebProofRequestStatus>(
    WebProofRequestStatus.idle,
  );

  useEffect(() => {
    webProofProvider.addEventListeners(
      ExtensionMessageType.ProofDone,
      ({ payload: { proof } }) => {
        console.log("useWebproof: ProofDone message", proof);
        setWebProof(proof);
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
    error,
    status,
    isIdle: status === WebProofRequestStatus.idle,
    isPending: status === WebProofRequestStatus.pending,
    isError: status === WebProofRequestStatus.error,
    isSuccess: status === WebProofRequestStatus.success,
    request: () => {
      setStatus(WebProofRequestStatus.pending);
      webProofProvider.requestWebProof(webProofRequest);
    },
  };
};
