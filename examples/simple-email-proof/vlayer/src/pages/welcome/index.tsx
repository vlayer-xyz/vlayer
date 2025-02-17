import { Link } from "react-router";

export const WelcomeScreen = () => {
  return (
    <div className="mt-5 flex justify-center">
      <Link to="connect-wallet" id="nextButton" data-testid="start-page-button">
        Start
      </Link>
    </div>
  );
};

export default WelcomeScreen;
