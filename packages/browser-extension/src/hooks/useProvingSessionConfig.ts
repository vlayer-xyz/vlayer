import { useSessionStorage } from "@vlayer/extension-hooks";
import { WebProverSessionConfig } from "../web-proof-commons";

export const useProvingSessionConfig = () => {
  const initialValue = {
    steps: [],
    notaryUrl: null,
    wsProxyUrl: null,
    logoUrl: null,
    jwtToken: null,
  };
  const [config] = useSessionStorage<WebProverSessionConfig>(
    "webProverSessionConfig",
    initialValue,
  );
  return [config];
};
