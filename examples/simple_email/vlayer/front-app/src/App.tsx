import { useState, useMemo } from "react";
import { createVlayerClient, preverifyEmail } from "@vlayer/sdk";
import { createWalletClient, custom } from "viem";
import { optimismSepolia } from "viem/chains";
import { getStrFromFile } from "./lib/utils";

import emailProofProver from "../../../out/EmailDomainProver.sol/EmailDomainProver";
import emailProofVerifier from "../../../out/EmailProofVerifier.sol/EmailDomainVerifier";

function App() {
  const [file, setFile] = useState<File | null>(null);
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
      if (!file) throw new Error("no_eml_file");

      setCurrentStep("Parsing EML...");

      const eml = await getStrFromFile(file);
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
        fileName: file?.name,
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
      setFile(null);
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
    <div className="min-h-screen flex items-center justify-center bg-gray-900">
      <div className="w-full max-w-md m-6">
        <h1 className="text-center mb-3 text-xl">
          Claim your @vlayer.xyz NFT badge
        </h1>
        <form
          onSubmit={handleSubmit}
          className="rounded-lg px-8 pt-6 pb-8 mb-4 border border-violet-600"
        >
          <div className="mb-6">
            <label
              className="block text-gray-200 text-sm font-bold mb-2"
              htmlFor="file"
            >
              EML File Upload
            </label>
            <input
              id="file"
              type="file"
              accept=".eml"
              className="file-input file-input-bordered file-input-primary w-full"
              onChange={(e) => e.target.files && setFile(e.target.files[0])}
              required
            />
          </div>

          <div className="flex items-center justify-center">
            <button type="submit" className="btn btn-primary w-full">
              {isSubmitting ? currentStep : "Connect & Claim NFT"}
            </button>
          </div>

          <p className="text-block text-center text-red-400 mt-5">{errorMsg}</p>
        </form>
      </div>
    </div>
  );
}

export default App;
