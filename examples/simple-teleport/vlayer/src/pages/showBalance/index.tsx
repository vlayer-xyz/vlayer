import { FormEvent, useEffect, useState } from "react";
import { useLocalStorage } from "usehooks-ts";
import { useNavigate } from "react-router";
import { ConnectWallet } from "../../shared/components/ConnectWallet";
import { getChainName } from "../../shared/lib/utils";

export const ShowBalancePage = () => {
  const navigate = useNavigate();
  const [holderAddress, setHolderAddress] = useState<`0x${string}` | null>(
    null,
  );
  const [balances, setBalances] = useState<string[]>([]);
  const [tokens, setTokens] = useState<
    { addr: string; chainId: string; blockNumber: string }[]
  >([]);
  const [isLoading, setIsLoading] = useState(true);
  const [proverResult] = useLocalStorage("proverResult", "");

  useEffect(() => {
    if (proverResult) {
      const [, owner, balances, tokens] = JSON.parse(proverResult) as [
        unknown,
        `0x${string}`,
        string[],
        { addr: string; chainId: string; blockNumber: string }[],
      ];
      setHolderAddress(owner);
      setBalances(balances);
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
        {tokens.map(
          (
            token: { addr: string; chainId: string; blockNumber: string },
            index: number,
          ) => (
            <div key={token.addr}>
              {getChainName(token.chainId)}: {balances[index]} (block:{" "}
              {token.blockNumber})
            </div>
          ),
        )}
      </div>
      <div className="mt-5 flex justify-center">
        <button type="submit" id="nextButton" disabled={isLoading}>
          {isLoading ? "Loading..." : "Generate Proof NFT"}
        </button>
      </div>
    </form>
  );
};
