import { FallbackProps } from "react-error-boundary";
import { AppError } from "../../errors";

export const ErrorBoundaryComponent = ({ error }: FallbackProps) => {
  const errorMsg =
    error instanceof AppError ? error.message : "Something went wrong";

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
