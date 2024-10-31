import { useState, useMemo } from "react";
import { createVlayerClient, preverifyEmail } from "@vlayer/sdk";
import { createWalletClient, custom } from "viem";
import { optimismSepolia } from "viem/chains";
import { getStrFromFile } from "../lib/utils";

import emailProofProver from "../../../../out/EmailDomainProver.sol/EmailDomainProver";
import emailProofVerifier from "../../../../out/EmailProofVerifier.sol/EmailDomainVerifier";

import EmlForm from "../components/EmlForm";

const EmlUploadForm = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [currentStep, setCurrentStep] = useState("");
  const [errorMsg, setErrorMsg] = useState("");

  const chain = optimismSepolia;

  const vlayer = useMemo(
    () =>
      createVlayerClient({
        url: import.meta.env.VITE_PROVER_URL,
      }),
    [],
  );

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    try {
      e.preventDefault();
      setIsSubmitting(true);
      setErrorMsg("");
      if (typeof window !== "undefined" && !window.ethereum)
        throw new Error("no_wallet_detected");

      const formData = new FormData(e.currentTarget);
      const emlFile = formData.get("emlFile") as File | null;
      if (!emlFile) throw new Error("no_eml_file");

      setCurrentStep("Parsing EML...");

      const eml = await getStrFromFile(emlFile);
      const email = await preverifyEmail(eml);

      setCurrentStep("Connecting to wallet...");

      const walletClient = createWalletClient({
        chain,
        transport: custom(window.ethereum),
      });
      await walletClient.switchChain({ id: chain.id });
      const [claimerAddr] = await walletClient.requestAddresses();

      console.log("Form submitted:", {
        verifierAddress: import.meta.env.VITE_VERIFIER_ADDR,
        proverAddress: import.meta.env.VITE_PROVER_ADDR,
        fileName: emlFile?.name,
        unverifiedEmail: eml,
        claimerAddr,
        email,
      });
      setCurrentStep("Sending to prover...");

      const { hash } = await vlayer.prove({
        address: import.meta.env.VITE_PROVER_ADDR,
        proverAbi: emailProofProver.abi,
        functionName: "main",
        args: [email, claimerAddr],
        chainId: chain.id,
      });

      setCurrentStep("Waiting for proof...");
      console.log("Waiting for proving result: ", hash);
      const result = await vlayer.waitForProvingResult({ hash });
      console.log("Response:", result);
      setCurrentStep("Verifying on-chain...");

      const txHash = await walletClient.writeContract({
        address: import.meta.env.VITE_VERIFIER_ADDR as `0x${string}`,
        abi: emailProofVerifier.abi,
        functionName: "verify",
        args: result,
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
