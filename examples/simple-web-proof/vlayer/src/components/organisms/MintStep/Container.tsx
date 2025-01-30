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
import { testPrivateKey, useTestPrivateKey } from "../../../utils";
import { ensureBalance } from "../../../utils/ethFaucet";

export const MintStep = () => {
  const navigate = useNavigate();
  const modalRef = useRef<HTMLDialogElement>(null);
  const [mintedHandle, setMintedHandle] = useState<string | null>(null);
  const [isMinting, setIsMinting] = useState(false);
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
      await ensureBalance(address as `0x${string}`, balance?.value ?? 0n);
      writeContract(writeContractArgs);
    }
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
    }
  }, [error]);

  return (
    <MintStepPresentational
      handleMint={handleMint}
      isMinting={isMinting}
      errorMsg={error?.message}
    />
  );
};
