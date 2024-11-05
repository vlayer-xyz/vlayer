import { useState, useMemo, useEffect } from "react";
import { createWalletClient, custom } from "viem";
import { optimismSepolia } from "viem/chains";
import useProver from "../hooks/useProver";
import { preverifyEmail } from "@vlayer/sdk";
import { getStrFromFile } from "../lib/utils";

import emailProofProver from "../../../../out/EmailDomainProver.sol/EmailDomainProver";
import emailProofVerifier from "../../../../out/EmailProofVerifier.sol/EmailDomainVerifier";

import EmlForm from "../components/EmlForm";

const EmlUploadForm = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [currentStep, setCurrentStep] = useState("");
  const [errorMsg, setErrorMsg] = useState("");
  const [claimerAddr, setClaimerAddr] = useState("");
  const chain = optimismSepolia;

  const walletClient = useMemo(
    () =>
      createWalletClient({
        chain,
        transport: custom(window.ethereum),
      }),
    [],
  );

  const { prove, proof, provingError } = useProver({
    addr: import.meta.env.VITE_PROVER_ADDR,
    abi: emailProofProver.abi,
    func: "main",
    chainId: chain.id,
  });

  const getClaimerAddr = async () => {
    if (typeof window !== "undefined" && !window.ethereum)
      throw new Error("no_wallet_detected");

    await walletClient.switchChain({ id: chain.id });
    const [addr] = await walletClient.requestAddresses();

    setClaimerAddr(addr);
    return addr;
  };

  const verifyProof = async () => {
    try {
      setCurrentStep("Verifying on-chain...");

      const txHash = await walletClient.writeContract({
        address: import.meta.env.VITE_VERIFIER_ADDR as `0x${string}`,
        abi: emailProofVerifier.abi,
        functionName: "verify",
        args: proof,
        chain,
        account: claimerAddr,
      });

      console.log({ txHash });
      setCurrentStep("Success!");
      setIsSubmitting(false);
      window.location.href = `${chain.blockExplorers.default.url}/tx/${txHash}`;
    } catch (err) {
      console.log({ err });
      setIsSubmitting(false);
      setErrorMsg(
        err?.shortMessage || err?.message || "Something went wrong, check logs",
      );
    }
  };

  const startProving = async (uploadedEmlFile: File, claimerAddr: string) => {
    setCurrentStep("Sending to prover...");

    const eml = await getStrFromFile(uploadedEmlFile);
    const email = await preverifyEmail(eml);
    await prove([email, claimerAddr]);
    setCurrentStep("Waiting for proof...");
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    try {
      e.preventDefault();
      setIsSubmitting(true);
      setErrorMsg("");

      const formData = new FormData(e.currentTarget);
      const emlFile = formData.get("emlFile") as File | null;
      if (!emlFile) throw new Error("no_eml_file");

      setCurrentStep("Connecting to wallet...");
      const addr = await getClaimerAddr();
      console.log("Form submitted:", {
        verifierAddress: import.meta.env.VITE_VERIFIER_ADDR,
        proverAddress: import.meta.env.VITE_PROVER_ADDR,
        fileName: emlFile?.name,
        claimerAddr: addr,
      });
      await startProving(emlFile, addr);
      setCurrentStep("Waiting for proof...");
    } catch (err) {
      console.log({ err });
      setIsSubmitting(false);
      setErrorMsg(
        err?.shortMessage || err?.message || "Something went wrong, check logs",
      );
    }
  };

  useEffect(() => {
    if (proof) verifyProof();
  }, [proof]);

  useEffect(() => {
    if (provingError) {
      setErrorMsg(provingError);
      setIsSubmitting(false);
    }
  }, [provingError]);

  return (
    <EmlForm
      isSubmitting={isSubmitting}
      handleSubmit={handleSubmit}
      errorMsg={errorMsg}
      currentStep={currentStep}
    />
  );
};

export default EmlUploadForm;
