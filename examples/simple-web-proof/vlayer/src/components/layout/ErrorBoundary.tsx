import { FallbackProps } from "react-error-boundary";

export const ErrorBoundaryComponent = ({ error }: FallbackProps) => {
  const errorMessage = error.message;
  console.error(errorMessage);

  if (errorMessage === "Already minted") {
    return (
      <div data-testid="Error display">
        <p style={{ color: "black" }}>
          NFT has been already minted fot this account.
        </p>
      </div>
    );
  }
  return (
    <div data-testid="Error display">
      <p style={{ color: "red" }}>Something went wrong</p>
    </div>
  );
};
