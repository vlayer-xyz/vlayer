import { CollectEmail } from "./Presentational";
import { useSearchParams, useNavigate } from "react-router";
import { useEffect } from "react";
import useExampleInbox from "../../shared/hooks/useExampleInbox";
import { getStepPath } from "../../app/router/steps";
import { STEP_KIND } from "../../app/router/types";

export const CollectEmailContainer = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const uniqueEmail = searchParams.get("uniqueEmail");
  const emailId = uniqueEmail?.split("@")[0];

  const { emlFetched } = useExampleInbox(emailId);

  useEffect(() => {
    if (emlFetched) {
      navigate(`/${getStepPath(STEP_KIND.MINT_NFT)}`);
    }
  }, [emlFetched]);

  return <CollectEmail />;
};
