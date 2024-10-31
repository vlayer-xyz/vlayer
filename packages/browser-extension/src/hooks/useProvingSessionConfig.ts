import { useLocalStorage } from "@vlayer/extension-hooks";
import { WebProverSessionConfig } from "../web-proof-commons";

export const useProvingSessionConfig = () => {
  return useLocalStorage<WebProverSessionConfig>("webProverSessionConfig", {
    steps: [],
    notaryUrl: null,
    wsProxyUrl: null,
    logoUrl: null,
  });
};
