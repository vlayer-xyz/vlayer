import { FormEvent } from "react";

export const WelcomeForm = ({
  networkChain,
  token,
  holderAddress,
  onSubmit,
  isLoading,
}: {
  networkChain: string;
  token: string;
  holderAddress: `0x${string}`;
  onSubmit: (e: FormEvent<HTMLFormElement>) => void;
  isLoading: boolean;
}) => {
  return (
    <form onSubmit={onSubmit}>
      <div className="mb-4 w-full block">
        <label
          htmlFor="networkChain"
          className="block text-sm font-medium mb-1 text-slate-900"
        >
          Chain
        </label>
        <select
          id="networkChain"
          className="w-full p-2 border border-gray-300 rounded-md bg-gray-50 text-slate-900"
          disabled
        >
          <option value={networkChain}>{networkChain}</option>
        </select>
      </div>
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
        />
      </div>
      <div className="mb-4 w-full block">
        <label
          htmlFor="token"
          className="block text-sm font-medium mb-1 text-slate-900"
        >
          Token
        </label>
        <select
          id="token"
          className="w-full p-2 border border-gray-300 rounded-md bg-gray-50 text-slate-900"
          disabled
        >
          <option value={token}>{token}</option>
        </select>
      </div>
      <div className="mt-5 flex justify-center">
        <button
          type="submit"
          id="nextButton"
          data-testid="start-page-button"
          disabled={isLoading}
        >
          {isLoading ? "Loading..." : "Show balance"}
        </button>
      </div>
    </form>
  );
};
