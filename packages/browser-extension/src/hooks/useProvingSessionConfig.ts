import { useSessionStorage } from "@vlayer/extension-hooks";
import { WebProverSessionConfig } from "../web-proof-commons";

export const useProvingSessionConfig = () => {
  const initialValue = {
    steps: [],
    notaryUrl: null,
    wsProxyUrl: null,
    logoUrl: null,
    token: undefined,
  };
  const [config] = useSessionStorage<WebProverSessionConfig>(
    "webProverSessionConfig",
    initialValue,
  );
  return [config];
};
