import { useProvingSessionConfig } from "hooks/useProvingSessionConfig";
import { isEmptyWebProverSessionConfig } from "../../web-proof-commons";

import * as React from "react";
import { LOADING } from "@vlayer/extension-hooks";
import { EmptyFlowCard } from "components/molecules/EmptyFlow";
import { HelpSection } from "components/organisms";
import { Steps } from "components/organisms";
export const SidePanelContent = () => {
  const [config] = useProvingSessionConfig();
  return (
    <>
      {config === LOADING ? (
        <div>Loading...</div>
      ) : isEmptyWebProverSessionConfig(config) ? (
        <EmptyFlowCard />
      ) : (
        <>
          <Steps />
          <HelpSection />
        </>
      )}
    </>
  );
};
