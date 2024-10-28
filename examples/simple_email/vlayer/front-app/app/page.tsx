"use client";

import { useState, useRef, useMemo } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { Upload, MailCheck } from "lucide-react";
import { getStrFromFile } from "@/lib/utils";
import { createVlayerClient, preverifyEmail, testHelpers } from "@vlayer/sdk";
import { createWalletClient, custom } from "viem";
import { useRouter } from "next/navigation";

import emailProofProver from "../../../out/EmailDomainProver.sol/EmailDomainProver";
import emailProofVerifier from "../../../out/EmailProofVerifier.sol/EmailDomainVerifier";

export default function Home() {
  const [errorMsg, setErrorMsg] = useState("");
  const [currentStep, setCurrentStep] = useState("Submitting...");
  const [claimerAddress, setClaimerAddress] = useState("");
  const [file, setFile] = useState(null);
  const [isDragging, setIsDragging] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const router = useRouter();

  const vlayer = useMemo(
    () =>
      createVlayerClient({
        url: process.env.NEXT_PUBLIC_PROVER_URL,
      }),
    [],
  );

  const handleDragOver = (e) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = () => {
    setIsDragging(false);
  };

  const handleDrop = (e) => {
    e.preventDefault();
    setIsDragging(false);

    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      setFile(e.dataTransfer.files[0]);
    }
  };

  const handleFileChange = (e) => {
    if (e.target.files && e.target.files[0]) {
      setFile(e.target.files[0]);
    }
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsSubmitting(true);
    setCurrentStep("Submitting...");
    setErrorMsg("");

    try {
      if (window && !window.ethereum) throw new Error("no_wallet_detected");

      if (!file) throw new Error("no_eml_file_uploaded");
      setCurrentStep("Parsing eml...");
      const eml = await getStrFromFile(file);
      const email = await preverifyEmail(eml);

      console.log("Form submitted:", {
        verifierAddress: process.env.NEXT_PUBLIC_VERIFIER_ADDR,
        proverAddress: process.env.NEXT_PUBLIC_PROVER_ADDR,
        fileName: file?.name,
        unverifiedEmail: eml,
        email,
      });
      setCurrentStep("Sending to prover...");

      const { hash } = await vlayer.prove({
        address: process.env.NEXT_PUBLIC_PROVER_ADDR,
        proverAbi: emailProofProver.abi,
        functionName: "main",
        args: [await preverifyEmail(eml), claimerAddress],
        chainId: Number(process.env.NEXT_PUBLIC_CHAIN_ID),
      });
      setCurrentStep("Waiting for proof...");
      console.log("Waiting for proving result: ", hash);
      const result = await vlayer.waitForProvingResult({ hash });
      console.log("Response:", result);
      setCurrentStep("Verifying on-chain...");

      const chain = testHelpers.getChainConfig(
        Number(process.env.NEXT_PUBLIC_CHAIN_ID),
      );

      if(!chain) throw new Error("wrong chain provided")

      const walletClient = createWalletClient({
        chain,
        transport: custom(window.ethereum!),
      });
      await walletClient.switchChain({ id: chain.id });
      const [account] = await walletClient.requestAddresses();
      setClaimerAddress(account ?? "");

      const txHash = await walletClient.writeContract({
        address: process.env.NEXT_PUBLIC_VERIFIER_ADDR as `0x${string}`,
        abi: emailProofVerifier.abi,
        functionName: "verify",
        args: result,
        chain,
        account,
      });

      console.log({ txHash });
      setFile(null);
      if (fileInputRef.current && fileInputRef.current.value) {
        fileInputRef.current.value = "";
      }
      setCurrentStep("Success!");
      router.push(`/success?txHash=${txHash}`);
    } catch (error) {
      console.error("Error submitting form:", error);
      setErrorMsg(
        error?.shortMessage ||
          error?.message ||
          "Something went wrong, check console",
      );
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="flex justify-center items-center min-h-screen p-4 bg-gray-950">
      <Card className="w-full max-w-md border-violet-500 bg-gray-900 text-white">
        <CardHeader>
          <CardTitle className="text-violet-400">
            Generate proof of <i className="text-white">@vlayer.xyz</i>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="space-y-2">
              <Label className="text-violet-300">Upload EML file</Label>
              <div
                onDragOver={handleDragOver}
                onDragLeave={handleDragLeave}
                onDrop={handleDrop}
                className={`border-2 border-dashed rounded-lg p-6 text-center cursor-pointer transition-all ${
                  isDragging
                    ? "border-violet-400 bg-violet-500/10"
                    : "border-violet-500 hover:border-violet-400 hover:bg-violet-500/5"
                }`}
              >
                <input
                  ref={fileInputRef}
                  type="file"
                  name="fileInput"
                  onChange={handleFileChange}
                  className="hidden"
                  id="fileInput"
                  required
                />
                <label htmlFor="fileInput" className="cursor-pointer">
                  <div className="flex flex-col items-center gap-2">
                    {file ? (
                      <MailCheck className="w-8 h-8 text-green-400" />
                    ) : (
                      <Upload className="w-8 h-8 text-violet-400" />
                    )}
                    <div className="text-sm text-gray-300">
                      {file
                        ? file.name
                        : "Drag and drop a file here, or click to select"}
                    </div>
                  </div>
                </label>
              </div>
            </div>

            <div className="space-y-2">
              <Label htmlFor="claimerAddr" className="text-violet-300">
                Address to claim NFT
              </Label>
              <Input
                id="claimerAddr"
                onChange={(e) => setClaimerAddress(e.target.value)}
                value={claimerAddress}
                className="bg-gray-800 border-violet-500 text-white placeholder:text-gray-400 focus:ring-violet-500 focus:border-violet-500"
                required
              />
            </div>

            <Button
              type="submit"
              disabled={isSubmitting || !file}
              className="w-full rounded-full bg-violet-600 hover:bg-violet-700 text-white font-medium py-2 px-4 transition-colors"
            >
              {isSubmitting ? (
                <div className="flex items-center justify-center gap-2">
                  <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
                  {currentStep}
                </div>
              ) : (
                "Submit"
              )}
            </Button>
            {errorMsg && <p className="text-center text-red-400">{errorMsg}</p>}
          </form>
        </CardContent>
      </Card>
    </div>
  );
}
