import { FormEvent } from "react";

export const HodlerForm = ({
  networkChain,
  token,
  holderAddress,
  onSubmit,
  isLoading,
  loadingLabel,
  submitLabel,
  isEditable,
  balance,
}: {
  networkChain: string;
  token: string;
  holderAddress: string;
  onSubmit: (e: FormEvent<HTMLFormElement>) => void | Promise<void>;
  isLoading: boolean;
  loadingLabel: string;
  submitLabel: string;
  isEditable: boolean;
  balance?: string;
}) => {
  return (
    <form
      onSubmit={(e) => {
        void onSubmit(e);
      }}
    >
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
          disabled={!isEditable}
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
          disabled={!isEditable}
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
          disabled={!isEditable}
        >
          <option value={token}>{token}</option>
        </select>
      </div>
      {balance && (
        <div className="mb-4 w-full block text-black">
          Average balance (between blocks {import.meta.env.VITE_START_BLOCK} and{" "}
          {import.meta.env.VITE_END_BLOCK}):
          <br />
          {balance}
        </div>
      )}
      <div className="mt-5 flex justify-center">
        <button type="submit" id="nextButton" disabled={isLoading}>
          {isLoading ? loadingLabel : submitLabel}
        </button>
      </div>
    </form>
  );
};
