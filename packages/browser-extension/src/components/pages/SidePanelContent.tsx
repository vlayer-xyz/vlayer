import { useProvingSessionConfig } from "hooks/useProvingSessionConfig";
import { WebProverSessionConfig } from "../../web-proof-commons";

import React, { useEffect } from "react";
import * as Sentry from "@sentry/react";
import { LOADING } from "@vlayer/extension-hooks";
import { EmptyFlowCard } from "components/molecules/EmptyFlow";
import { HelpSection } from "components/organisms";
import { Steps } from "components/organisms";
import { ErrorCallout } from "components/organisms/ErrorCallout";
import { useCleanStorageOnClose } from "hooks/useCleanStorageOnClose";
import { useCloseSidePanelOnRequest } from "hooks/useCloseSidePanelOnRequest";
import { useConnectToBackground } from "hooks/useConnectToBackground";
import { match } from "ts-pattern";
import { useResetTlsnSessionOnNewWebproofRequest } from "hooks/useResetTlsnSessionOnRequest";
export const SidePanelContent = ({
  config,
}: {
  config?: WebProverSessionConfig | typeof LOADING;
}) => {
  return match(config)
    .with(LOADING, () => <div>Loading...</div>)
    .with(undefined, () => <EmptyFlowCard />)
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
  useConnectToBackground();
  useResetTlsnSessionOnNewWebproofRequest();
  const [config] = useProvingSessionConfig();
  useEffect(() => {
    if (config !== LOADING && Sentry.isInitialized()) {
      Sentry.setContext("WebProverSessionConfig", {
        notaryUrl: config.notaryUrl,
        wsProxyUrl: config.wsProxyUrl,
      });
    }
  }, [config]);

  return <SidePanelContent config={config} />;
};
