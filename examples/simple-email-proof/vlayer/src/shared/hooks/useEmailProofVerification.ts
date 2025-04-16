import { useState, useEffect } from "react";
import { useWriteContract, useWaitForTransactionReceipt } from "wagmi";
import { useCallProver, useWaitForProvingResult } from "@vlayer/react";
import { preverifyEmail } from "@vlayer/sdk";
import proverSpec from "../../../../out/EmailDomainProver.sol/EmailDomainProver";
import verifierSpec from "../../../../out/EmailProofVerifier.sol/EmailDomainVerifier";
import { AbiStateMutability, ContractFunctionArgs } from "viem";
import { useNavigate } from "react-router";
import debug from "debug";

const log = debug("vlayer:email-proof-verification");

class NoProofError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "NoProofError";
  }
}

enum ProofVerificationStep {
  MINT = "Mint",
  SENDING_TO_PROVER = "Sending to prover...",
  WAITING_FOR_PROOF = "Waiting for proof...",
  VERIFYING_ON_CHAIN = "Verifying on-chain...",
  DONE = "Done!",
}

export const useEmailProofVerification = () => {
  const navigate = useNavigate();
  const [currentStep, setCurrentStep] = useState<ProofVerificationStep>(
    ProofVerificationStep.MINT,
  );

  const {
    writeContract,
    data: txHash,
    error: verificationError,
    status,
  } = useWriteContract();

  const { status: onChainVerificationStatus } = useWaitForTransactionReceipt({
    hash: txHash,
  });

  const { callProver, data: proofHash } = useCallProver({
    address: import.meta.env.VITE_PROVER_ADDRESS,
    proverAbi: proverSpec.abi,
    functionName: "main",
  });

  const { data: proof, error: provingError } =
    useWaitForProvingResult(proofHash);

  const verifyProofOnChain = () => {
    setCurrentStep(ProofVerificationStep.VERIFYING_ON_CHAIN);

    if (!proof) {
      throw new NoProofError("No proof available to verify on-chain");
    }

    const contractArgs: Parameters<typeof writeContract>[0] = {
      address: import.meta.env.VITE_VERIFIER_ADDRESS,
      abi: verifierSpec.abi,
      functionName: "verify",
      args: proof as unknown as ContractFunctionArgs<
        typeof verifierSpec.abi,
        AbiStateMutability,
        "verify"
      >,
    };

    writeContract(contractArgs);
  };

  const startProving = async (emlContent: string) => {
    setCurrentStep(ProofVerificationStep.SENDING_TO_PROVER);

    const email = await preverifyEmail({
      mimeEmail: emlContent,
      dnsResolverUrl: import.meta.env.VITE_DNS_SERVICE_URL,
      token: import.meta.env.VITE_VLAYER_API_TOKEN,
    });
    await callProver([email]);
    setCurrentStep(ProofVerificationStep.WAITING_FOR_PROOF);
  };

  useEffect(() => {
    if (proof) {
      log("proof", proof);
      void verifyProofOnChain();
    }
  }, [proof]);

  useEffect(() => {
    if (status === "success" && proof) {
      setCurrentStep(ProofVerificationStep.DONE);
      const proofArray = proof as unknown[];
      void navigate(
        `/success?txHash=${txHash}&domain=${String(proofArray[3])}&recipient=${String(proofArray[2])}`,
      );
    }
  }, [status]);

  return {
    currentStep,
    txHash,
    onChainVerificationStatus,
    verificationError,
    provingError,
    startProving,
  };
};
