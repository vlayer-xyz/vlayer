import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import {
  useWriteContract,
  useWaitForTransactionReceipt,
  useAccount,
  useBalance,
} from "wagmi";
import { useLocalStorage } from "usehooks-ts";

import webProofProofVerifier from "../../../../../out/WebProofVerifier.sol/WebProofVerifier.json";
import { MintStepPresentational } from "./Presentational";
import { ensureBalance } from "../../../utils/ethFaucet";
import { AlreadyMintedError } from "../../../errors";

export const MintStep = () => {
  const navigate = useNavigate();
  const modalRef = useRef<HTMLDialogElement>(null);
  const [mintedHandle, setMintedHandle] = useState<string | null>(null);
  const [isMinting, setIsMinting] = useState(false);
  // Using mintingError state to throw error in useEffect because ErrorBoundary does not catch errors from async functions like handleMint
  const [mintingError, setMintingError] = useState<Error | null>(null);
  const [proverResult] = useLocalStorage("proverResult", "");
  const { address } = useAccount();
  const { data: balance } = useBalance({ address });
  const { writeContract, data: txHash, error } = useWriteContract();
  const { status } = useWaitForTransactionReceipt({
    hash: txHash,
  });

  useEffect(() => {
    console.log("proverResult", proverResult);
    if (proverResult) {
      setMintedHandle(JSON.parse(proverResult)[1]);
    }
    modalRef.current?.showModal();
  }, [proverResult]);

  const handleMint = async () => {
    setIsMinting(true);
    if (!proverResult) {
      return;
    }

    const proofData = JSON.parse(proverResult);

    console.log("proofData", proofData);
    console.log("address", address);

    console.log(
      "import.meta.env.VITE_VERIFIER_ADDRESS",
      import.meta.env.VITE_VERIFIER_ADDRESS,
    );

    const writeContractArgs: Parameters<typeof writeContract>[0] = {
      address: import.meta.env.VITE_VERIFIER_ADDRESS as `0x${string}`,
      abi: webProofProofVerifier.abi,
      functionName: "verify",
      args: proofData,
    };

    try {
      await ensureBalance(address as `0x${string}`, balance?.value ?? 0n);
    } catch (error) {
      setMintingError(error as Error);
    }
    console.log("writeContractArgs", writeContractArgs);
    writeContract(writeContractArgs);
  };

  useEffect(() => {
    if (status === "success") {
      setIsMinting(false);
      navigate(`/success?tx=${txHash}&handle=${mintedHandle}`);
    }
  }, [status]);

  useEffect(() => {
    if (error) {
      setIsMinting(false);
      if (error.message.includes("User has already minted a TwitterNFT")) {
        console.log("AlreadyMintedError");
        console.log(error);
        throw new AlreadyMintedError();
      } else {
        throw error;
      }
    }
  }, [error]);

  useEffect(() => {
    if (mintingError) {
      setIsMinting(false);
      throw mintingError;
    }
  }, [mintingError]);

  return (
    <MintStepPresentational handleMint={handleMint} isMinting={isMinting} />
  );
};
