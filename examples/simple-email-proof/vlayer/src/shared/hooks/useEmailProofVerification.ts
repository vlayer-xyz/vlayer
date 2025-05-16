import { useState, useEffect } from "react";
import {
  useWriteContract,
  useWaitForTransactionReceipt,
  useAccount,
  useBalance,
} from "wagmi";
import {
  useCallProver,
  useWaitForProvingResult,
  useChain,
} from "@vlayer/react";
import { preverifyEmail } from "@vlayer/sdk";
import proverSpec from "../../../../out/EmailDomainProver.sol/EmailDomainProver";
import verifierSpec from "../../../../out/EmailProofVerifier.sol/EmailDomainVerifier";
import { AbiStateMutability, ContractFunctionArgs } from "viem";
import { useNavigate } from "react-router";
import debug from "debug";
import {
  AlreadyMintedError,
  NoProofError,
  CallProverError,
  UseChainError,
  PreverifyError,
} from "../errors/appErrors";
import { ensureBalance } from "../lib/ethFaucet";

const log = debug("vlayer:email-proof-verification");

enum ProofVerificationStep {
  MINT = "Mint",
  SENDING_TO_PROVER = "Sending to prover...",
  WAITING_FOR_PROOF = "Waiting for proof...",
  VERIFYING_ON_CHAIN = "Verifying on-chain...",
  DONE = "Done!",
}

export const useEmailProofVerification = () => {
  const { address } = useAccount();
  const { data: balance } = useBalance({ address });
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

  const { chain, error: chainError } = useChain(
    import.meta.env.VITE_CHAIN_NAME,
  );
  if (chainError) {
    throw new UseChainError(chainError);
  }

  const {
    callProver,
    data: proofHash,
    error: callProverError,
  } = useCallProver({
    address: import.meta.env.VITE_PROVER_ADDRESS,
    proverAbi: proverSpec.abi,
    functionName: "main",
    gasLimit: Number(import.meta.env.VITE_GAS_LIMIT),
    chainId: chain?.id,
  });

  if (callProverError) {
    throw new CallProverError(callProverError.message);
  }

  const { data: proof, error: provingError } =
    useWaitForProvingResult(proofHash);

  if (provingError) {
    throw new CallProverError(provingError.message);
  }

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

    await ensureBalance(address as `0x${string}`, balance?.value ?? 0n);

    writeContract(contractArgs);
  };

  const [preverifyError, setPreverifyError] = useState<Error | null>(null);
  const startProving = async (emlContent: string) => {
    setCurrentStep(ProofVerificationStep.SENDING_TO_PROVER);

    try {
      const email = await preverifyEmail({
        mimeEmail: emlContent,
        dnsResolverUrl: import.meta.env.VITE_DNS_SERVICE_URL,
        token: import.meta.env.VITE_VLAYER_API_TOKEN,
      });
      await callProver([email]);
    } catch (error) {
      setPreverifyError(error as Error);
    }
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

  useEffect(() => {
    if (verificationError) {
      if (verificationError.message.includes("already been minted")) {
        throw new AlreadyMintedError();
      }
      throw new Error(verificationError.message);
    }
  }, [verificationError]);

  useEffect(() => {
    if (preverifyError) {
      throw new PreverifyError(preverifyError.message);
    }
  }, [preverifyError]);

  return {
    currentStep,
    txHash,
    onChainVerificationStatus,
    verificationError,
    provingError,
    startProving,
  };
};
