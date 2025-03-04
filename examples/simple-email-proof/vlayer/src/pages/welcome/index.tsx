import { Link } from "react-router";
import { getStepPath } from "../../app/router/steps";
import { STEP_KIND } from "../../app/router/types";

export const WelcomePage = () => {
  return (
    <div className="mt-5 flex justify-center">
      <Link
        to={`/${getStepPath(STEP_KIND.CONNECT_WALLET)}`}
        id="nextButton"
        data-testid="start-page-button"
      >
        Start
      </Link>
    </div>
  );
};
