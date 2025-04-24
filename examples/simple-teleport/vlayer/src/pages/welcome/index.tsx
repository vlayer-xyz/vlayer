import { FormEvent, useEffect, useState } from "react";
import { useProver } from "../../shared/hooks/useProver";
import { useNavigate } from "react-router";
import { getStepPath } from "../../app/router/steps";
import { StepKind } from "../../app/router/types";
import { HodlerForm } from "../../shared/forms/HodlerForm";
import { ConnectWallet } from "../../shared/components/ConnectWallet";
export const WelcomePage = () => {
  const navigate = useNavigate();
  const [isLoading, setIsLoading] = useState(false);
  const defaultTokenHolder = import.meta.env
    .VITE_DEFAULT_TOKEN_HOLDER as `0x${string}`;
  const { callProver, result } = useProver();

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsLoading(true);
    const formData = new FormData(e.target as HTMLFormElement);
    const holderAddress = formData.get("holderAddress") as `0x${string}`;
    const tokensToCheck = JSON.parse(import.meta.env.VITE_TOKENS_TO_CHECK) as {
      addr: string;
      chainId: string;
      blockNumber: string;
    }[];
    void callProver([holderAddress, tokensToCheck]);
  };

  useEffect(() => {
    if (result) {
      void navigate(getStepPath(StepKind.showBalance));
      setIsLoading(false);
    }
  }, [result]);

  if (!defaultTokenHolder) {
    return <ConnectWallet />;
  }

  return (
    <HodlerForm
      holderAddress={defaultTokenHolder}
      onSubmit={handleSubmit}
      isLoading={isLoading}
      loadingLabel="Loading..."
      submitLabel="Show cross-chain balance"
      isEditable={true}
    />
  );
};
