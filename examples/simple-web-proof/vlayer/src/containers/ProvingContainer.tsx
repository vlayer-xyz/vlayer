import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import { useSimpleWebProof } from "../hooks/useSimpleWebProof";
import { StartProving } from "../components/StartProving";
import { useAppKitAccount } from "@reown/appkit/react";

export const ProvingContainer = () => {
  const navigate = useNavigate();
  const { address } = useAppKitAccount();
  const [disabled, setDisabled] = useState(false);
  const modalRef = useRef<HTMLDialogElement>(null);

  const { requestWebProof, webProof, callProver, isPending, result } =
    useSimpleWebProof();

  useEffect(() => {
    if (webProof) {
      callProver([webProof, address]);
    }
  }, [webProof]);

  useEffect(() => {
    if (result) {
      navigate("/minting");
    }
  }, [result]);

  useEffect(() => {
    modalRef.current?.showModal();
  }, []);

  return (
    <StartProving
      requestWebProof={requestWebProof}
      isPending={isPending}
      disabled={disabled}
      setDisabled={setDisabled}
    />
  );
};
