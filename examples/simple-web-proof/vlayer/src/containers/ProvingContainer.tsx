import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import { Modal } from "../components/Modal";
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
    console.log("webProof", webProof);
    if (webProof) {
      callProver([webProof, address]);
    }
  }, [webProof]);

  useEffect(() => {
    console.log("result", result);
    if (result) {
      navigate("/minting");
    }
  }, [result]);

  useEffect(() => {
    modalRef.current?.showModal();
  }, []);

  return (
    <Modal backUrl="/connect-wallet">
      <StartProving
        requestWebProof={requestWebProof}
        isPending={isPending}
        disabled={disabled}
        setDisabled={setDisabled}
      />
    </Modal>
  );
};
