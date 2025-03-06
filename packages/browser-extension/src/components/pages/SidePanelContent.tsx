import { useProvingSessionConfig } from "hooks/useProvingSessionConfig";
import {
  isEmptyWebProverSessionConfig,
  WebProverSessionConfig,
} from "../../web-proof-commons";

import * as React from "react";
import { LOADING } from "@vlayer/extension-hooks";
import { EmptyFlowCard } from "components/molecules/EmptyFlow";
import { HelpSection } from "components/organisms";
import { Steps } from "components/organisms";
import { ErrorCallout } from "components/organisms/ErrorCallout";
import { useCleanStorageOnClose } from "hooks/useCleanStorageOnClose";
import { useCloseSidePanelOnRequest } from "hooks/useCloseSidePanelOnRequest";
import { match } from "ts-pattern";

export const SidePanelContent = ({
  config,
}: {
  config: WebProverSessionConfig | typeof LOADING;
}) => {
  return match(config)
    .with(LOADING, () => <div>Loading...</div>)
    .when(isEmptyWebProverSessionConfig, () => <EmptyFlowCard />)
    .otherwise(() => (
      <>
        <Steps />
        <ErrorCallout />
        <HelpSection />
      </>
    ));
};

export const SidePanelContainer = () => {
  useCleanStorageOnClose();
  useCloseSidePanelOnRequest();
  const [config] = useProvingSessionConfig();
  return <SidePanelContent config={config} />;
};
