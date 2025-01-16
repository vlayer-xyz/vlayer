import React from "react";

import { Button, Loading } from "react-daisyui";

export function CallProverButton({
  disabled,
  onClick,
  isLoading,
}: {
  disabled: boolean;
  onClick: () => void;
  isLoading: boolean;
}) {
  return (
    <Button
      disabled={disabled}
      color="primary"
      size="lg"
      id="callProver"
      onClick={onClick}
      className={`${disabled ? "!text-gray-400" : "hover:shadow-primary/50"}`}
    >
      <div className="flex items-center justify-center gap-2">
        {isLoading ? <Loading variant="infinity" /> : <ProveIcon />}
        Prove
      </div>
    </Button>
  );
}

const ProveIcon = () => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      className="h-5 w-5"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
      />
    </svg>
  );
};
