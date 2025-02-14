import { Link } from "react-router";
import styles from "./welcome.module.css";

export const WelcomeScreen = () => {
  return (
    <div className={styles.startButton}>
      <Link to="connect-wallet" id="nextButton" data-testid="start-page-button">
        Start
      </Link>
    </div>
  );
};

export default WelcomeScreen;
