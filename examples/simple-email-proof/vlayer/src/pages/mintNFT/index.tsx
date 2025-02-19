import { useState } from "react";
import { useEmailProofVerification } from "../../shared/hooks/useEmailProofVerification";
import { MintNFT } from "./Presentational";

export const MintNFTContainer = () => {
  const [copyCode, setCopyCode] = useState(false);
  const [emlFile, setEmlFile] = useState<File | undefined>(undefined);

  const { currentStep, startProving } = useEmailProofVerification();

  const handleProving = () => {
    if (emlFile) {
      startProving(emlFile);
    }
  };

  return (
    <MintNFT
      copyCode={copyCode}
      setCopyCode={setCopyCode}
      emlFile={emlFile}
      setEmlFile={setEmlFile}
      currentStep={currentStep}
      handleProving={handleProving}
    />
  );
};
