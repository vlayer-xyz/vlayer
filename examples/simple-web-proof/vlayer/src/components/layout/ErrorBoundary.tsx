import { FallbackProps } from "react-error-boundary";
import { match } from "ts-pattern";

export const ErrorBoundaryComponent = ({ error }: FallbackProps) => {
  const errorMessage = error.message;
  console.error(errorMessage);

  return match(error.name)
    .with("AlreadyMintedError", () => {
      return (
        <div data-testid="Error display">
          <p style={{ color: "black" }}>
            NFT has already been minted for this account.
          </p>
        </div>
      );
    })
    .otherwise(() => {
      return (
        <div data-testid="Error display">
          <p style={{ color: "red" }}>Something went wrong</p>
        </div>
      );
    });
};
