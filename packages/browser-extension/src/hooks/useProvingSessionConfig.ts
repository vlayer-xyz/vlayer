import { LOADING, useLocalStorage } from "@vlayer/extension-hooks";
import { WebProverSessionConfig } from "../web-proof-commons";

export const useProvingSessionConfig = () => {
  const initialValue = {
    steps: [],
    notaryUrl: null,
    wsProxyUrl: null,
    logoUrl: null,
  };
  const [config] = useLocalStorage<WebProverSessionConfig>(
    "webProverSessionConfig",
    initialValue,
  );
  return [config === LOADING ? initialValue : config];
};
