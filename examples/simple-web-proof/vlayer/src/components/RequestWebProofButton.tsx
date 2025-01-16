import { Loading, Button } from "react-daisyui";
import React from "react";

export function RequestWebProofButton({
  onClick,
  isLoading,
  hasWebProof,
}: {
  onClick: () => void;
  isLoading: boolean;
  hasWebProof: boolean;
}) {
  return (
    <Button color="primary" size="lg" id="requestWebProof" onClick={onClick}>
      {isLoading ? (
        <div className="flex items-center justify-center gap-3">
          <Loading variant="infinity" size="sm" />
          <span>Generating Web Proof...</span>
        </div>
      ) : (
        <RequestWebProofButtonContent hasWebProof={hasWebProof} />
      )}
    </Button>
  );
}

const RequestWebProofIcon = () => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      className="h-5 w-5"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
    >
      <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z" />
    </svg>
  );
};

const CheckmarkIcon = () => {
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
        d="M5 13l4 4L19 7"
      />
    </svg>
  );
};

const RequestWebProofButtonContent = ({
  hasWebProof,
}: {
  hasWebProof: boolean;
}) => {
  return (
    <div className="flex items-center justify-center gap-2">
      {hasWebProof ? <CheckmarkIcon /> : <RequestWebProofIcon />}
      {hasWebProof ? "X.com Proof Generated" : "Request X.com Proof"}
    </div>
  );
};
