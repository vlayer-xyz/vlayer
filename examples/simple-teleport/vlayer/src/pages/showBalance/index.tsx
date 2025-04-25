import { FormEvent, useEffect, useState } from "react";
import { useLocalStorage } from "usehooks-ts";
import { useNavigate } from "react-router";
import { ConnectWallet } from "../../shared/components/ConnectWallet";
import { getChainName, parseProverResult } from "../../shared/lib/utils";

export const ShowBalancePage = () => {
  const navigate = useNavigate();
  const [holderAddress, setHolderAddress] = useState<`0x${string}` | null>(
    null,
  );
  const [tokensToProve, setTokens] = useState<
    { addr: string; chainId: string; blockNumber: string; balance: string }[]
  >([]);
  const [isLoading, setIsLoading] = useState(true);
  const [proverResult] = useLocalStorage("proverResult", "");

  useEffect(() => {
    if (proverResult) {
      const [, owner, tokens] = parseProverResult(proverResult);
      setHolderAddress(owner);
      setTokens(tokens);
      setIsLoading(false);
    }
  }, [proverResult]);

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    void navigate(`/confirm-mint`);
  };

  if (!holderAddress) {
    return <ConnectWallet />;
  }

  return (
    <form onSubmit={handleSubmit}>
      <div className="mb-4 w-full block">
        <label
          htmlFor="holderAddress"
          className="block text-sm font-medium mb-1 text-slate-900"
        >
          Address or ENS of token holder:
        </label>
        <input
          name="holderAddress"
          type="text"
          defaultValue={holderAddress}
          className="w-full p-2 border border-gray-300 rounded-md bg-gray-50 text-slate-900"
          disabled
        />
      </div>
      <div className="p-4 bg-slate-100 rounded-lg text-slate-800">
        {tokensToProve.map((token) => (
          <div key={token.addr}>
            {getChainName(token.chainId)}: {token.balance} (block:{" "}
            {token.blockNumber})
          </div>
        ))}
      </div>
      <div className="mt-5 flex justify-center">
        <button type="submit" id="nextButton" disabled={isLoading}>
          {isLoading ? "Loading..." : "Generate Proof NFT"}
        </button>
      </div>
    </form>
  );
};
