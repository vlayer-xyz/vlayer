import { FormEvent } from "react";

export const HodlerForm = ({
  holderAddress,
  onSubmit,
  isLoading,
  loadingLabel,
  submitLabel,
  isEditable,
}: {
  holderAddress: string;
  onSubmit: (e: FormEvent<HTMLFormElement>) => void;
  isLoading: boolean;
  loadingLabel: string;
  submitLabel: string;
  isEditable: boolean;
}) => {
  return (
    <form onSubmit={onSubmit}>
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
      <div className="mt-5 flex justify-center">
        <button type="submit" id="nextButton" disabled={isLoading}>
          {isLoading ? loadingLabel : submitLabel}
        </button>
      </div>
    </form>
  );
};
