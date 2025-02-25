import { Link } from "react-router";
import { getStepBackUrl, STEP_KIND } from "../../../app/router/steps";

export const NextButton = () => {
  return (
    <Link
      to={getStepBackUrl(STEP_KIND.MINT_NFT)}
      id="nextButton"
      data-testid="connect-wallet-button"
    >
      Next
    </Link>
  );
};
