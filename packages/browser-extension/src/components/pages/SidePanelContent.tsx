import { useProvingSessionConfig } from "hooks/useProvingSessionConfig";
import { isEmptyWebProverSessionConfig } from "../../web-proof-commons";

import * as React from "react";
import { LOADING } from "@vlayer/extension-hooks";
import { EmptyFlowCard } from "components/molecules/EmptyFlow";
import { HelpSection } from "components/organisms";
import { Steps } from "components/organisms";
import { ErrorCallout } from "components/organisms/ErrorCallout";
export const SidePanelContent = () => {
  const [config] = useProvingSessionConfig();

  if (config === LOADING) {
    return <div>Loading...</div>;
  }

  if (isEmptyWebProverSessionConfig(config)) {
    return <EmptyFlowCard />;
  }

  return (
    <>
      <Steps />
      <ErrorCallout />
      <HelpSection />
    </>
  );
};
