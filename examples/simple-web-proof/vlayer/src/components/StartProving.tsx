import { useEffect, useRef, useState } from "react";
import { useSimpleWebProof } from "../hooks/useSimpleWebProof";
import { useAppKitAccount } from "@reown/appkit/react";

const extensionId = "jbchhcgphfokabmfacnkafoeeeppjmpl";

const isMobile =
  /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
    navigator.userAgent,
  );

const isSupportedBrowser = () => {
  const isChromium = !!(window as any).chrome;
  const isBrave = navigator.brave?.isBrave?.() || false;
  return isChromium || isBrave;
};

const checkExtensionInstalled = async () => {
  try {
    await chrome.runtime.sendMessage(extensionId, {
      message: "ping",
    });
    return true;
  } catch {
    return false;
  }
};

export const StartProving = () => {
  const { address } = useAppKitAccount();
  const [disabled, setDisabled] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const modalRef = useRef<HTMLDialogElement>(null);

  const { requestWebProof, webProof, callProver, isPending } =
    useSimpleWebProof();

  useEffect(() => {
    if (webProof) {
      callProver([webProof, address]);
    }
  }, [webProof]);

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
    <>
      <button className="btn" onClick={() => modalRef.current?.showModal()}>
        Start
      </button>
      <dialog id="my_modal_3" className="modal" ref={modalRef}>
        <div className="modal-box bg-white rounded-2xl ">
          <form method="dialog">
            <Link
              to="/connect-wallet"
              className="absolute left-3 text-black top-3 text-xs font-normal"
            >
              Back
            </Link>
            <button className="btn btn-sm btn-circle btn-ghost absolute right-3 text-black top-3">✕</button>
          </form>
          <ul className="steps w-full">
            <li className="step step-primary text-black text-xs">
              Connect Wallet
            </li>
            <li className="step step-primary text-black text-xs font-bold">
              Get data from X
            </li>
            <li className="step text-black text-xs">Mint NFT</li>
          </ul>
          <h3 className="mt-7 text-center text-black text-3xl font-bold ">
            X NFT
          </h3>
          <p className="py-4 text-gray-500">
            Open vlayer browser extension and follow instructions in order to produce the Proof of X account ownership.
          </p>
          <div className="mt-7 flex justify-center">
            <button
              disabled={disabled}
              className="btn w-[188px] px-4 bg-[#915bf8] rounded-lg border-none text-white hover:bg-[#915bf8]/80 hover:text-white"
              onClick={() => {
                console.log("open extension");
                requestWebProof();
                setDisabled(true);
              }}
            >
              {isPending ? "Proving in progress..." : "Open Extension"}
            </button>
          </div>
          {error && <p className="text-red-500 w-full block">{error}</p>}
        </div>
      </dialog>
    </>
  );
};
