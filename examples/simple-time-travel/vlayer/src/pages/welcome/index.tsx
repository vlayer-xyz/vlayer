import { FormEvent, useEffect, useState } from "react";
import { useAccount } from "wagmi";
import { useProver } from "../../shared/hooks/useProver";
import { useNavigate } from "react-router";
import { getStepPath } from "../../app/router/steps";
import { StepKind } from "../../app/router/types";
import { HodlerForm } from "../../shared/forms/HodlerForm";
import { ConnectWallet } from "../../shared/components/ConnectWallet";
export const WelcomePage = () => {
  const navigate = useNavigate();
  const [isLoading, setIsLoading] = useState(false);
  const { address } = useAccount();
  const networkChain = import.meta.env.VITE_CHAIN_NAME;
  const token = "USDC";

  const { callProver, result } = useProver();

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsLoading(true);
    const formData = new FormData(e.target as HTMLFormElement);
    const holderAddress = formData.get("holderAddress") as `0x${string}`;
    await callProver([holderAddress]);
  };

  useEffect(() => {
    if (result) {
      void navigate(getStepPath(StepKind.showBalance));
      setIsLoading(false);
    }
  }, [result]);

  if (!address) {
    return <ConnectWallet />;
  }

  return (
    <HodlerForm
      networkChain={networkChain}
      token={token}
      holderAddress={import.meta.env.VITE_PROVER_ERC20_HOLDER_ADDR}
      onSubmit={handleSubmit}
      isLoading={isLoading}
      loadingLabel="Loading..."
      submitLabel="Show balance"
      isEditable={true}
    />
  );
};
