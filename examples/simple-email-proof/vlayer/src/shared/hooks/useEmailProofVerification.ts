import { useState, useEffect } from "react";
import {
  useWriteContract,
  useWaitForTransactionReceipt,
  useAccount,
} from "wagmi";
import { useCallProver, useWaitForProvingResult } from "@vlayer/react";
import { preverifyEmail } from "@vlayer/sdk";
import { usePrivateKey, getStrFromFile } from "../lib/utils";
import proverSpec from "../../../../out/EmailDomainProver.sol/EmailDomainProver";
import verifierSpec from "../../../../out/EmailProofVerifier.sol/EmailDomainVerifier";
import { privateKeyToAccount } from "viem/accounts";
import { AbiStateMutability, ContractFunctionArgs, type Address } from "viem";
import { useNavigate } from "react-router";
class NoProofError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "NoProofError";
  }
}

enum ProofVerificationStep {
  START = "Start",
  SENDING_TO_PROVER = "Sending to prover...",
  WAITING_FOR_PROOF = "Waiting for proof...",
  VERIFYING_ON_CHAIN = "Verifying on-chain...",
  DONE = "Done!",
}

export const useEmailProofVerification = () => {
  const navigate = useNavigate();
  const [currentStep, setCurrentStep] = useState<ProofVerificationStep>(
    ProofVerificationStep.START,
  );
  const { address: connectedAddr } = useAccount();

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

  const verifyProofOnChain = async () => {
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

    if (usePrivateKey) {
      await writeContract({
        ...contractArgs,
        account: privateKeyToAccount(
          import.meta.env.VITE_PRIVATE_KEY as `0x${string}`,
        ),
      });
    } else {
      await writeContract(contractArgs);
    }
  };

  const startProving = async (uploadedEmlFile: File) => {
    setCurrentStep(ProofVerificationStep.SENDING_TO_PROVER);
    const claimerAddr = usePrivateKey
      ? privateKeyToAccount(import.meta.env.VITE_PRIVATE_KEY as `0x${string}`)
          .address
      : (connectedAddr as Address);

    const eml = await getStrFromFile(uploadedEmlFile);
    const email = await preverifyEmail(
      eml,
      import.meta.env.VITE_DNS_SERVICE_URL,
    );
    await callProver([email, claimerAddr]);
    setCurrentStep(ProofVerificationStep.WAITING_FOR_PROOF);
  };

  useEffect(() => {
    if (proof) {
      verifyProofOnChain();
    }
  }, [proof]);

  useEffect(() => {
    if (status === "success") {
      setCurrentStep(ProofVerificationStep.DONE);
      navigate(`/success?txHash=${txHash}`);
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
