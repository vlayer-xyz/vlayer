import { useState, useEffect, FormEvent } from "react";
import EmlForm from "../components/EmlForm";
import { useEmailProofVerification } from "../hooks/useEmailProofVerification";

const EmlUploadForm = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");
  const [successMsg, setSuccessMsg] = useState("");

  const {
    currentStep,
    txHash,
    onChainVerificationStatus,
    verificationError,
    provingError,
    startProving,
  } = useEmailProofVerification();

  const handleError = (err: unknown) => {
    setIsSubmitting(false);
    setSuccessMsg("");
    if (err instanceof Error) {
      setErrorMsg(
        err.message.includes("email taken")
          ? "Email already used. Try a different one or redeploy contracts"
          : err.message,
      );
    } else {
      setErrorMsg("Something went wrong, check logs");
    }
  };

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsSubmitting(true);
    setErrorMsg("");

    try {
      const formData = new FormData(e.currentTarget);
      const emlFile = formData.get("emlFile") as File;
      if (!emlFile) {
        throw new Error("no_eml_file");
      }

      await startProving(emlFile);
    } catch (err) {
      handleError(err);
    }
  };

  useEffect(() => {
    if (onChainVerificationStatus === "success" && txHash) {
      setIsSubmitting(false);
      setSuccessMsg(`Verified: ${txHash.slice(0, 4)}...${txHash.slice(-4)}`);
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
