import { FallbackProps } from "react-error-boundary";
import { AppError } from "../../errors";
import styles from "./ErrorBoundary.module.css";
import { useNavigate } from "react-router";

export const StepErrorBoundaryComponent = ({ error }: FallbackProps) => {
  const errorMsg =
    error instanceof AppError ? error.message : "Something went wrong";

  return (
    <div className={styles.container}>
      <div className={styles.image}>
        <img src="../../../error-illustration.png" />
      </div>
      <div className={styles.errorMsg}>{errorMsg}</div>
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
  console.error(error.message);

  const errorMsg =
    error instanceof AppError ? error.message : "Something went wrong";

  const navigate = useNavigate();

  const handleStartAgain = () => {
    navigate("/");
    window.location.reload();
  };

  return (
    <div className={styles.background}>
      <div className="modal-box bg-white rounded-2xl items-center justify-center">
        <div className={styles.errorScreen}>
          <div className={styles.image}>
            <img src="../../../error-illustration.png" />
          </div>
          <div className={styles.errorMsg}>{errorMsg}</div>
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
