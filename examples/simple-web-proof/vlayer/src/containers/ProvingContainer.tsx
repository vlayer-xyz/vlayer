import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import { Modal } from "../components/Modal";
import { useSimpleWebProof } from "../hooks/useSimpleWebProof";
import { StartProving } from "../components/StartProving";
import { useAppKitAccount } from "@reown/appkit/react";
import {
  isMobile,
  isSupportedBrowser,
  checkExtensionInstalled,
} from "../utils";

export const ProvingContainer = () => {
  const navigate = useNavigate();
  const { address } = useAppKitAccount();
  const [disabled, setDisabled] = useState(true);
  const [error, setError] = useState<string | undefined>(undefined);
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

  const isExtensionReady = async () => {
    if (isMobile) {
      setError("Mobile browsers are not supported");
      return;
    }
    if (!isSupportedBrowser()) {
      setError("Unsupported browser. Please try Chrome based browsers.");
      return;
    }

    const isInstalled = await checkExtensionInstalled();
    if (!isInstalled) {
      setError("Please install vlayer extension and try again");
      return;
    }

    setDisabled(false);
  };

  useEffect(() => {
    isExtensionReady();
    modalRef.current?.showModal();
  }, []);

  return (
    <Modal backUrl="/connect-wallet">
      <StartProving
        requestWebProof={requestWebProof}
        isPending={isPending}
        disabled={disabled}
        setDisabled={setDisabled}
        error={error}
      />
    </Modal>
  );
};
