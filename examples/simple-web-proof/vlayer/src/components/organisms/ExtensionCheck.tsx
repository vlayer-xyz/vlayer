import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import { Modal } from "../layout/Modal";
import { InstallExtension } from "./InstallExtension";
import {
  isMobile,
  isSupportedBrowser,
  checkExtensionInstalled,
} from "../../utils";

export const ExtensionCheck = () => {
  const navigate = useNavigate();
  const [extensionOk, setExtensionOk] = useState(false);
  const [error, setError] = useState<string | undefined>(undefined);
  const modalRef = useRef<HTMLDialogElement>(null);

  const isExtensionReady = async () => {
    if (isMobile) {
      setError("Mobile browsers are not supported");
      setExtensionOk(false);
      return;
    }
    if (!isSupportedBrowser()) {
      setError("Unsupported browser. Please try Chrome based browsers.");
      setExtensionOk(false);
      return;
    }

    const isInstalled = await checkExtensionInstalled();
    if (!isInstalled) {
      setError("Please install vlayer extension and try again");
      setExtensionOk(false);
      return;
    }

    setExtensionOk(true);
  };

  useEffect(() => {
    isExtensionReady();
    modalRef.current?.showModal();
  }, []);

  useEffect(() => {
    if (extensionOk) {
      navigate("/start-proving");
    }
  }, [extensionOk]);

  // Check if extension installed every 5 seconds
  useEffect(() => {
    const interval = setInterval(isExtensionReady, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <Modal>
      <InstallExtension error={error} />
    </Modal>
  );
};
