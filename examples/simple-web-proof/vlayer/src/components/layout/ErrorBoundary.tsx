import { FallbackProps } from "react-error-boundary";
import { AppError } from "../../errors";
import styles from "./ErrorBoundary.module.css";

export const ErrorBoundaryComponent = ({ error }: FallbackProps) => {
  const errorMsg =
    error instanceof AppError ? error.message : "Something went wrong";

  return (
    <div className={styles.container}>
      <div className={styles.image}>
        <img src="../../../error-illustration.png" />
      </div>
      <div className={styles.errorMsg}>{errorMsg}</div>
      <div className={styles.additionalText}>
        Click the button below to refresh
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
