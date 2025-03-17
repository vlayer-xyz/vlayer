import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import { useTwitterAccountProof } from "../../../hooks/useSimpleWebProof";
import { ProveStepPresentational } from "./Presentational";
import { useAccount } from "wagmi";

export const ProveStep = () => {
  const navigate = useNavigate();
  const { address } = useAccount();
  const [disabled, setDisabled] = useState(false);
  const modalRef = useRef<HTMLDialogElement>(null);

  const { requestWebProof, webProof, callProver, isPending, result, error } =
    useTwitterAccountProof();

  useEffect(() => {
    if (webProof) {
      callProver([webProof, address]);
    }
  }, [webProof]);

  useEffect(() => {
    if (result) {
      navigate("/mint");
    }
  }, [result]);

  useEffect(() => {
    modalRef.current?.showModal();
  }, []);

  return (
    <ProveStepPresentational
      requestWebProof={requestWebProof}
      isPending={isPending}
      disabled={disabled}
      setDisabled={setDisabled}
      errorMsg={error?.message}
    />
  );
};
