import { useEffect, useRef, useState } from "react";
import { Link, useNavigate } from "react-router";
import { useWriteContract, useWaitForTransactionReceipt } from "wagmi";
import webProofProofVerifier from "../../../out/WebProofVerifier.sol/WebProofVerifier.json";
import { privateKeyToAccount } from "viem/accounts";

const usePrivateKey =
  !import.meta.env.VITE_USE_WINDOW_ETHEREUM_TRANSPORT &&
  Boolean(import.meta.env.VITE_PRIVATE_KEY);

console.log({ usePrivateKey });

export const Minting = () => {
  const navigate = useNavigate();
  const modalRef = useRef<HTMLDialogElement>(null);
  const [mintedHandle, setMintedHandle] = useState<string | null>(null);
  const [isMinting, setIsMinting] = useState(false);

  const { writeContract, data: txHash, error } = useWriteContract();
  const { status } = useWaitForTransactionReceipt({
    hash: txHash,
  });

  useEffect(() => {
    const storedData = localStorage.getItem("proverResult");
    if (storedData) {
      setMintedHandle(JSON.parse(storedData)[1]);
    }
    modalRef.current?.showModal();
  }, []);

  const handleMint = () => {
    setIsMinting(true);
    const storedData = localStorage.getItem("proverResult");
    if (!storedData) {
      return;
    }

    const proofData = JSON.parse(storedData);

    const writeContractArgs: Parameters<typeof writeContract>[0] = {
      address: import.meta.env.VITE_VERIFIER_ADDRESS as `0x${string}`,
      abi: webProofProofVerifier.abi,
      functionName: "verify",
      args: proofData,
    };

    if (usePrivateKey) {
      writeContract({
        ...writeContractArgs,
        account: privateKeyToAccount(
          import.meta.env.VITE_PRIVATE_KEY as `0x${string}`,
        ),
      });
    } else {
      writeContract(writeContractArgs);
    }
  };

  useEffect(() => {
    if (status === "success") {
      setIsMinting(false);
      navigate("/success");
    }
  }, [status]);

  return (
    <>
      <button className="btn" onClick={() => modalRef.current?.showModal()}>
        Start
      </button>
      <dialog id="my_modal_3" className="modal" ref={modalRef}>
        <div className="modal-box bg-white rounded-2xl ">
          <form method="dialog">
            <Link
              to="/start-proving"
              className="absolute left-3 text-black top-3 text-xs font-normal"
            >
              Back
            </Link>
            <button className="btn btn-sm btn-circle btn-ghost absolute right-3 text-black top-3">✕</button>
          </form>
          <ul className="steps w-full">
            <li className="step step-primary text-black text-xs">Connect Wallet</li>
            <li className="step step-primary text-black text-xs">Get data from X</li>
            <li className="step step-primary text-black text-xs font-bold">Mint NFT</li>
          </ul>
          <h3 className="mt-7 text-center text-black text-3xl font-bold ">
            X NFT
          </h3>
          <p className="py-4 text-gray-500">
            You are all set to mint your unique @{mintedHandle} X NFT, a true reflection of your verified identity.
          </p>
          <div className="mt-7 flex justify-center">
            <button
              disabled={isMinting}
              className="btn w-[188px] px-4 bg-[#915bf8] rounded-lg border-none text-white hover:bg-[#915bf8]/80 hover:text-white"
              onClick={handleMint}
            >
              {isMinting ? "Minting..." : "Start Minting"}
            </button>
          </div>
          {error && (
            <p className="text-red-500 mt-5">Error: {error?.message}</p>
          )}
        </div>
      </dialog>
    </>
  );
};
