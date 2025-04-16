import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import { useTwitterAccountProof } from "../../../hooks/useTwitterAccountProof";
import { ProveStepPresentational } from "./Presentational";
import { useAccount } from "wagmi";

export const ProveStep = () => {
  const navigate = useNavigate();
  const { address } = useAccount();
  const [disabled, setDisabled] = useState(false);
  const modalRef = useRef<HTMLDialogElement>(null);

  const {
    requestWebProof,
    webProof,
    callProver,
    isPending,
    isCallProverPending,
    result,
  } = useTwitterAccountProof();

  useEffect(() => {
    if (webProof && !isCallProverPending) {
      void callProver([webProof, address]);
    }
  }, [webProof, address, callProver, isCallProverPending]);

  useEffect(() => {
    if (result) {
      void navigate("/mint");
    }
  }, [result, navigate]);

  useEffect(() => {
    modalRef.current?.showModal();
  }, []);

  return (
    <ProveStepPresentational
      requestWebProof={requestWebProof}
      isPending={isPending}
      disabled={disabled}
      setDisabled={setDisabled}
    />
  );
};
