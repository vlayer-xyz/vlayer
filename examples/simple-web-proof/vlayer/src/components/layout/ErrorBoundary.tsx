import { FallbackProps } from "react-error-boundary";
import { AppError } from "../../errors";
import styles from "./ErrorBoundary.module.css";
import { useNavigate } from "react-router";
import { TlsnProveError } from "@vlayer/sdk";

export const StepErrorBoundaryComponent = ({ error }: FallbackProps) => {
  console.error("Step error: ", error);

  const errorMsg =
    error instanceof AppError ? error.message : "Something went wrong";

  return (
    <div className={styles.container}>
      <div className={styles.image}>
        <img src="/error-illustration.png" alt="Error illustration" />
      </div>
      <div className={styles.errorMsg} data-testid="Error display">
        {errorMsg}
      </div>
      <div className={styles.additionalText}>
        Click the button below to refresh.
      </div>
      <button
        className={styles.button}
        onClick={() => window.location.reload()}
      >
        Refresh
      </button>
    </div>
  );
};

export const AppErrorBoundaryComponent = ({ error }: FallbackProps) => {
  console.error("App error: ", error);

  const errorMsg =
    error instanceof AppError || error instanceof TlsnProveError
      ? error.message
      : "Something went wrong";

  const navigate = useNavigate();

  const handleStartAgain = () => {
    void navigate("/");
    window.location.reload();
  };

  return (
    <div className={styles.background}>
      <div className="modal-box bg-white rounded-2xl items-center justify-center">
        <div className={styles.errorScreen}>
          <div className={styles.image}>
            <img src="/error-illustration.png" alt="Error illustration" />
          </div>
          <div className={styles.errorMsg} data-testid="Error display">
            {errorMsg}
          </div>
          <div className={styles.additionalText}>
            Click the button below or try again later.
          </div>
          <button className={styles.button} onClick={handleStartAgain}>
            Start again
          </button>
        </div>
      </div>
    </div>
  );
};
