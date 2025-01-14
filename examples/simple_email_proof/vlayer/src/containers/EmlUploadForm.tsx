import { useState, useEffect, FormEvent } from "react";
import {
  useWriteContract,
  useWaitForTransactionReceipt,
  useAccount,
} from "wagmi";
import { useCallProver, useWaitForProvingResult } from "@vlayer/react";
import { preverifyEmail } from "@vlayer/sdk";
import { getStrFromFile } from "../lib/utils";
import proverSpec from "../../../out/EmailDomainProver.sol/EmailDomainProver";
import verifierSpec from "../../../out/EmailProofVerifier.sol/EmailDomainVerifier";
import EmlForm from "../components/EmlForm";
import { privateKeyToAccount } from "viem/accounts";
import { AbiStateMutability, ContractFunctionArgs, type Address } from "viem";

const usePrivateKey =
  !import.meta.env.VITE_USE_WINDOW_ETHEREUM_TRANSPORT ||
  Boolean(import.meta.env.VITE_PRIVATE_KEY);

const EmlUploadForm = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [currentStep, setCurrentStep] = useState("");
  const [errorMsg, setErrorMsg] = useState("");
  const [successMsg, setSuccessMsg] = useState("");

  const { address } = useAccount();

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

  const handleError = (err: unknown) => {
    setIsSubmitting(false);
    setSuccessMsg("");
    if (err instanceof Error) {
      if ("shortMessage" in err) {
        setErrorMsg(err.shortMessage as string);
      }
      if (err?.message?.includes("email taken")) {
        setErrorMsg(
          "Email already used. Try a different one or redeploy contracts",
        );
      } else {
        setErrorMsg(err.message);
      }
    } else {
      setErrorMsg("Something went wrong, check logs");
    }
  };

  const verifyProofOnChain = async () => {
    try {
      setCurrentStep("Verifying on-chain...");

      if (proof == null) {
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
        writeContract({
          ...contractArgs,
          account: privateKeyToAccount(
            import.meta.env.VITE_PRIVATE_KEY as `0x${string}`,
          ),
        });
      } else {
        writeContract(contractArgs);
      }
    } catch (err) {
      handleError(err);
    }
  };

  const finishProving = async () => {
    setCurrentStep("Success!");
    setIsSubmitting(false);
    if (txHash) {
      setSuccessMsg(`Verified: ${txHash.slice(0, 4)}...${txHash.slice(-4)}`);
    }
  };

  const startProving = async (uploadedEmlFile: File, claimerAddr: Address) => {
    setCurrentStep("Sending to prover...");

    const eml = await getStrFromFile(uploadedEmlFile);
    const email = await preverifyEmail(eml);
    await callProver([email, claimerAddr]);
    setCurrentStep("Waiting for proof...");
  };

  const claimerAddr = () => {
    if (usePrivateKey) {
      return privateKeyToAccount(
        import.meta.env.VITE_PRIVATE_KEY as `0x${string}`,
      ).address;
    }

    return address as Address;
  };

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsSubmitting(true);
    setErrorMsg("");

    const formData = new FormData(e.currentTarget);
    const emlFile = formData.get("emlFile") as File | null;
    if (!emlFile) {
      throw new Error("no_eml_file");
    }

    setCurrentStep("Connecting to wallet...");

    await startProving(emlFile, claimerAddr());
    setCurrentStep("Waiting for proof...");
  };

  useEffect(() => {
    if (proof) {
      verifyProofOnChain();
    }
  }, [proof]);

  useEffect(() => {
    if (onChainVerificationStatus === "success") {
      finishProving();
    }
  }, [onChainVerificationStatus]);

  useEffect(() => {
    if (verificationError) {
      handleError(verificationError);
    }
  }, [verificationError]);

  useEffect(() => {
    if (provingError) {
      handleError("Cannot finalize proving, check logs");
    }
  }, [provingError]);

  return (
    <EmlForm
      isSubmitting={isSubmitting}
      handleSubmit={handleSubmit}
      errorMsg={errorMsg}
      successMsg={successMsg}
      currentStep={currentStep}
    />
  );
};

export default EmlUploadForm;
