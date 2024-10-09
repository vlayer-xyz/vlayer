import { useLocalStorage } from "@vlayer/extension-hooks";
import { WebProverSessionConfig } from "@vlayer/web-proof-commons";

export const useProvingSessionConfig = () => {
  return useLocalStorage<WebProverSessionConfig | undefined>(
    "webProverSessionConfig",
    undefined,
  );
};
