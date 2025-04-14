import { FormEvent, useEffect, useState } from "react";
import verifierSpec from "../../../../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import { useLocalStorage } from "usehooks-ts";
import { useWriteContract } from "wagmi";
import { useNavigate } from "react-router";
import { HodlerForm } from "../../shared/forms/HodlerForm";
import { ConnectWallet } from "../../shared/components/ConnectWallet";

export const ShowBalancePage = () => {
  const navigate = useNavigate();
  const {
    writeContract,
    data: txHash,
    status,
    error: mintError,
  } = useWriteContract();
  const [holderAddress, setHolderAddress] = useState<`0x${string}` | null>(
    null,
  );
  const [balance, setBalance] = useState<string | undefined>(undefined);
  const [isLoading, setIsLoading] = useState(false);
  const networkChain = import.meta.env.VITE_CHAIN_NAME;
  const token = "USDC";
  const [proverResult] = useLocalStorage("proverResult", "");

  useEffect(() => {
    if (txHash && status === "success") {
      console.log("Claimed", txHash);
      void navigate(`/success?txHash=${txHash}`);
    }
  }, [txHash, status]);

  useEffect(() => {
    if (proverResult) {
      const [, owner, balance] = JSON.parse(proverResult) as [
        unknown,
        `0x${string}`,
        string,
      ];
      setHolderAddress(owner);
      setBalance(balance);
    }
  }, [proverResult]);

  useEffect(() => {
    if (mintError) {
      console.error("Mint error", mintError);
    }
  }, [mintError]);

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const [proof, owner, balance] = JSON.parse(proverResult) as [
      unknown,
      `0x${string}`,
      string,
    ];
    setIsLoading(true);
    writeContract({
      address: import.meta.env.VITE_VERIFIER_ADDRESS,
      abi: verifierSpec.abi,
      functionName: "claim",
      //@ts-expect-error proof is unknown @Artur fix this
      args: [proof, owner, BigInt(balance)],
    });
  };

  if (!holderAddress) {
    return <ConnectWallet />;
  }

  return (
    <HodlerForm
      networkChain={networkChain}
      token={token}
      holderAddress={holderAddress}
      onSubmit={handleSubmit}
      isLoading={isLoading}
      balance={balance}
      loadingLabel="Minting..."
      submitLabel="Generate Proof NFT"
      isEditable={false}
    />
  );
};
