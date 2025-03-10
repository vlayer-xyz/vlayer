import { useEmailProofVerification } from "../../shared/hooks/useEmailProofVerification";
import { MintNFT } from "./Presentational";
import { useLocalStorage } from "usehooks-ts";

export const MintNFTContainer = () => {
  const [emlFile] = useLocalStorage("emlFile", "");

  const { currentStep, startProving } = useEmailProofVerification();

  const handleProving = () => {
    if (emlFile) {
      startProving(emlFile);
    }
  };

  return <MintNFT currentStep={currentStep} handleProving={handleProving} />;
};
