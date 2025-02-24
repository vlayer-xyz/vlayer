import { Link } from "react-router";

export const NextButton = () => {
  return (
    <Link to="/mint-nft" id="nextButton" data-testid="connect-wallet-button">
      Next
    </Link>
  );
};
