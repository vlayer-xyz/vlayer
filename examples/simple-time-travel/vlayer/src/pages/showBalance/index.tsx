import { FormEvent, useEffect, useState } from "react";
import verifierSpec from "../../../../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import { useLocalStorage } from "usehooks-ts";
import { useAccount, useBalance, useWriteContract } from "wagmi";
import { useNavigate } from "react-router";
import { HodlerForm } from "../../shared/forms/HodlerForm";
import { ConnectWallet } from "../../shared/components/ConnectWallet";
import { AlreadyMintedError } from "../../shared/errors/appErrors";
import { ensureBalance } from "../../shared/lib/ethFaucet";

export const ShowBalancePage = () => {
  const { address } = useAccount();
  const { data: currentBalance } = useBalance({ address });
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
      if (mintError.message.includes("already been minted")) {
        throw new AlreadyMintedError();
      } else if (mintError.message.includes("User rejected the request")) {
        setIsLoading(false);
        console.log("User rejected the request");
      } else {
        throw new Error(mintError.message);
      }
    }
  }, [mintError]);

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const [proof, owner, balance] = JSON.parse(proverResult) as [
      unknown,
      `0x${string}`,
      string,
    ];
    setIsLoading(true);
    await ensureBalance(address as `0x${string}`, currentBalance?.value ?? 0n);
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
      loadingLabel="Please sign transaction"
      submitLabel="Generate Proof NFT"
      isEditable={false}
    />
  );
};
