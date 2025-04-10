import { CollectEmail } from "./Presentational";
import { useSearchParams, useNavigate } from "react-router";
import { useEffect } from "react";
import useExampleInbox from "../../shared/hooks/useExampleInbox";
import { getStepPath } from "../../app/router/steps";
import { StepKind } from "../../app/router/types";

export const CollectEmailContainer = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const uniqueEmail = searchParams.get("uniqueEmail");
  const emailId = uniqueEmail?.split("@")[0];

  const { emlFetched } = useExampleInbox(emailId);

  useEffect(() => {
    if (emlFetched) {
      void navigate(`/${getStepPath(StepKind.mintNft)}`);
    }
  }, [emlFetched, navigate]);

  return <CollectEmail />;
};
