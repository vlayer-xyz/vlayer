import { FormEvent, useEffect, useState } from "react";
import verifierSpec from "../../../../out/SimpleTeleportVerifier.sol/SimpleTeleportVerifier";
import { useLocalStorage } from "usehooks-ts";
import { useAccount, useWriteContract } from "wagmi";
import { useNavigate } from "react-router";
import { ConnectWallet } from "../../shared/components/ConnectWallet";

export const ConfirmMintPage = () => {
  const { address } = useAccount();
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
  const [isLoading, setIsLoading] = useState(false);
  const [proverResult] = useLocalStorage("proverResult", "");
  const tokensToCheck = JSON.parse(import.meta.env.VITE_TOKENS_TO_CHECK) as {
    addr: string;
    chainId: string;
    blockNumber: string;
  }[];

  useEffect(() => {
    if (txHash && status === "success") {
      void navigate(`/success?txHash=${txHash}`);
    }
  }, [txHash, status]);

  useEffect(() => {
    if (proverResult) {
      const [, owner] = JSON.parse(proverResult) as [
        unknown,
        `0x${string}`,
        string,
      ];
      setHolderAddress(owner);
    }
  }, [proverResult]);

  useEffect(() => {
    if (mintError) {
      console.error("Mint error", mintError);
    }
  }, [mintError]);

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const [proof, owner, balances, tokens] = JSON.parse(proverResult) as [
      unknown,
      `0x${string}`,
      string[],
      { addr: string; chainId: string; blockNumber: string }[],
    ];
    setIsLoading(true);
    writeContract({
      address: import.meta.env.VITE_VERIFIER_ADDRESS,
      abi: verifierSpec.abi,
      functionName: "claim",
      //@ts-expect-error proof is unknown
      args: [proof, owner, balances, tokens],
    });
  };

  if (!holderAddress) {
    return <ConnectWallet />;
  }

  return (
    <form onSubmit={handleSubmit}>
      <p className="desc w-full text-center">
        NFT of holding USDC across {tokensToCheck.length} chains
      </p>
      <div className="mb-4 w-full block">
        <label
          htmlFor="holderAddress"
          className="block text-sm font-medium mb-1 text-slate-900"
        >
          You will mint NFT for wallet:
        </label>
        <input
          name="holderAddress"
          type="text"
          defaultValue={holderAddress}
          className="w-full p-2 border border-gray-300 rounded-md bg-gray-50 text-slate-900"
          disabled
        />
      </div>
      <div className="mb-4 w-full block">
        <label
          htmlFor="minterAddress"
          className="block text-sm font-medium mb-1 text-slate-900"
        >
          Transaction will be sent from connected wallet:
        </label>
        <input
          name="minterAddress"
          type="text"
          defaultValue={address}
          className="w-full p-2 border border-gray-300 rounded-md bg-gray-50 text-slate-900"
          disabled
        />
      </div>
      <div className="mt-5 flex justify-center">
        <button type="submit" id="nextButton" disabled={isLoading}>
          {isLoading ? "Minting..." : "Mint token"}
        </button>
      </div>
    </form>
  );
};
