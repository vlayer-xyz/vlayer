import { Link } from "react-router";
import { getStepPath, STEP_KIND } from "../../../app/router/steps";

export const NextButton = () => {
  return (
    <Link
      to={getStepPath(STEP_KIND.MINT_NFT)}
      id="nextButton"
      data-testid="connect-wallet-button"
    >
      Next
    </Link>
  );
};
