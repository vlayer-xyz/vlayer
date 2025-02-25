import { Link } from "react-router";
import { getStepBackUrl, STEP_KIND } from "../../app/router/steps";

export const WelcomePage = () => {
  return (
    <div className="mt-5 flex justify-center">
      <Link
        to={getStepBackUrl(STEP_KIND.CONNECT_WALLET)}
        id="nextButton"
        data-testid="start-page-button"
      >
        Start
      </Link>
    </div>
  );
};
