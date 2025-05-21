import { FormEvent, useEffect, useMemo, useState } from "react";
import verifierSpec from "../../../../out/SimpleTeleportVerifier.sol/SimpleTeleportVerifier";
import { useLocalStorage } from "usehooks-ts";
import { useAccount, useBalance, useWriteContract } from "wagmi";
import { useNavigate } from "react-router";
import { ConnectWallet } from "../../shared/components/ConnectWallet";
import { parseProverResult, tokensToProve } from "../../shared/lib/utils";
import { AlreadyMintedError } from "../../shared/errors/appErrors";
import { Chain, optimismSepolia } from "viem/chains";
import { match } from "ts-pattern";
export const ConfirmMintPage = () => {
  const { address, chain } = useAccount();
  const { data: balance } = useBalance({ address: address as `0x${string}` });
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

  useEffect(() => {
    if (txHash && status === "success") {
      void navigate(`/success?txHash=${txHash}`);
    }
  }, [txHash, status]);

  useEffect(() => {
    if (proverResult) {
      const [, owner] = parseProverResult(proverResult);
      setHolderAddress(owner);
    }
  }, [proverResult]);

  useEffect(() => {
    if (mintError) {
      if (mintError.message.includes("already been minted")) {
        throw new AlreadyMintedError();
      }
      throw new Error(mintError.message);
    }
  }, [mintError]);

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const [proof, owner, tokens] = parseProverResult(proverResult);
    setIsLoading(true);
    writeContract({
      address: import.meta.env.VITE_VERIFIER_ADDRESS,
      abi: verifierSpec.abi,
      functionName: "claim",
      //@ts-expect-error proof is unknown
      args: [proof, owner, tokens],
    });
  };
  // estimated price for Sepolia verification tx
  const enoughBalance = balance?.value && balance.value > 3000000000000000n;

  if (!holderAddress) {
    return <ConnectWallet />;
  }

  return (
    <form onSubmit={handleSubmit}>
      <p className="desc w-full text-center">
        NFT of holding USDC across {tokensToProve.length} chains
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
      {!enoughBalance && chain && <FaucetInfo chain={chain} />}
    </form>
  );
};

const getFaucetUrl = (chainId: number) => {
  return match(chainId)
    .with(
      optimismSepolia.id,
      () => "https://cloud.google.com/application/web3/faucet/ethereum/sepolia",
    )
    .otherwise(() => null);
};

const FaucetInfo = ({ chain }: { chain: Chain }) => {
  const faucet = useMemo(() => getFaucetUrl(chain.id), [chain.id]);
  return (
    <p className="text-red-400 text-center mt-4">
      Insufficient balance in your wallet. <br />
      {faucet ? (
        <>
          Please fund your account with{" "}
          <a href={faucet} target="_blank" className="font-bold">
            {chain.name} Faucet
          </a>
        </>
      ) : (
        <>
          Please fill your wallet with {chain.nativeCurrency.name} from
          {chain.name}
        </>
      )}
    </p>
  );
};
