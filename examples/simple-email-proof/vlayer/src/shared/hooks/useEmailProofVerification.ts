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

export const useEmailProofVerification = () => {
  const [currentStep, setCurrentStep] = useState("");
  const { address: connectedAddr } = useAccount();

  const {
    writeContract,
    data: txHash,
    error: verificationError,
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

  console.log({ proof, provingError });

  const verifyProofOnChain = async () => {
    setCurrentStep("Verifying on-chain...");

    if (!proof) {
      throw new Error("no_proof_to_verify");
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
    setCurrentStep("Sending to prover...");
    const claimerAddr = usePrivateKey
      ? privateKeyToAccount(import.meta.env.VITE_PRIVATE_KEY as `0x${string}`)
          .address
      : (connectedAddr as Address);

    const eml = await getStrFromFile(uploadedEmlFile);
    const email = await preverifyEmail(eml, "http://127.0.0.1:3002/dns-query");
    await callProver([email, claimerAddr]);
    setCurrentStep("Waiting for proof...");
  };

  useEffect(() => {
    if (proof) {
      verifyProofOnChain();
    }
  }, [proof]);

  return {
    currentStep,
    txHash,
    proof,
    onChainVerificationStatus,
    verificationError,
    provingError,
    startProving,
    verifyProofOnChain,
  };
};
