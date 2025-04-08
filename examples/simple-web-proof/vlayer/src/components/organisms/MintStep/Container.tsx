import { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";
import {
  useWriteContract,
  useWaitForTransactionReceipt,
  useAccount,
  useBalance,
} from "wagmi";
import { z } from "zod";

import { useLocalStorage } from "usehooks-ts";

import webProofProofVerifier from "../../../../../out/WebProofVerifier.sol/WebProofVerifier";
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

  const proofDataSchema = z.tuple([
    z.string(),
    z.string(),
    z
      .string()
      .regex(/^0x[0-9a-fA-F]+$/, "Must be a hex string starting with 0x"),
  ]);

  useEffect(() => {
    if (proverResult) {
      const mintedHandle = proverResult[1];
      setMintedHandle(mintedHandle);
    }
    modalRef.current?.showModal();
  }, [proverResult]);

  const handleMint = async () => {
    setIsMinting(true);
    if (!proverResult) {
      return;
    }

    let proofData;

    try {
      proofData = proofDataSchema.parse(JSON.parse(proverResult));
    } catch {
      setMintingError(new Error("Invalid proverResult format"));
      return;
    }

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

    writeContract(writeContractArgs);
  };

  useEffect(() => {
    if (status === "success") {
      setIsMinting(false);
      void navigate(`/success?tx=${txHash}&handle=${mintedHandle}`);
    }
  }, [status, txHash, mintedHandle, navigate]);

  useEffect(() => {
    if (error) {
      setIsMinting(false);
      if (error.message.includes("User has already minted a TwitterNFT")) {
        throw new AlreadyMintedError();
      } else {
        throw new Error(error.message);
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
    <MintStepPresentational
      handleMint={() => void handleMint()}
      isMinting={isMinting}
    />
  );
};
