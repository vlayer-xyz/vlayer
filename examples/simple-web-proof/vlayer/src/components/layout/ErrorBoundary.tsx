import { FallbackProps } from "react-error-boundary";
import { KnownAppError } from "../../errors";

export const ErrorBoundaryComponent = ({ error }: FallbackProps) => {
  console.error(error.message);

  const errorMsg =
    error instanceof KnownAppError ? error.message : "Something went wrong";

  return (
    <div
      data-testid="Error display"
      role="alert"
      className="alert alert-error m-2"
      style={{ width: "300px" }}
    >
      <span>{errorMsg}</span>
    </div>
  );
};
