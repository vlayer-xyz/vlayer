import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import { useWriteContract, useWaitForTransactionReceipt } from "wagmi";

import webProofProofVerifier from "../../../out/WebProofVerifier.sol/WebProofVerifier.json";
import { Modal } from "../components/Modal";
import { Minting } from "../components/Minting";
import { testPrivateKey, useTestPrivateKey } from "../utils";

export const MintingContainer = () => {
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

    if (useTestPrivateKey) {
      writeContract({
        ...writeContractArgs,
        account: testPrivateKey,
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
    <Modal backUrl="/start-proving">
      <Minting
        mintedHandle={mintedHandle ?? ""}
        handleMint={handleMint}
        isMinting={isMinting}
        errorMsg={error?.message}
      />
    </Modal>
  );
};
