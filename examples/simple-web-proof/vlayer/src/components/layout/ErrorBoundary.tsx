import { FallbackProps } from "react-error-boundary";
import { KnownAppError } from "../../errors";

export const ErrorBoundaryComponent = ({ error }: FallbackProps) => {
  console.error(error.message);

  if (error instanceof KnownAppError) {
    return (
      <div data-testid="Error display">
        <p style={{ color: "red" }}>{error.name}</p>
        <pre style={{ color: "red" }}>{error.message}</pre>
      </div>
    );
  } else {
    return (
      <div data-testid="Error display">
        <p style={{ color: "red" }}>Something went wrong</p>
      </div>
    );
  }
};
