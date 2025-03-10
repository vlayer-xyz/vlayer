import { Link } from "react-router";
import { getStepPath } from "../../app/router/steps";
import { StepKind } from "../../app/router/types";

export const WelcomePage = () => {
  return (
    <div className="mt-5 flex justify-center">
      <Link
        to={`/${getStepPath(StepKind.connectWallet)}`}
        id="nextButton"
        data-testid="start-page-button"
      >
        Start
      </Link>
    </div>
  );
};
