import { Link } from "react-router";

export const NextButton = ({ path }: { path: string }) => {
  return (
    <Link to={path} id="nextButton" data-testid="connect-wallet-button">
      Next
    </Link>
  );
};
