import { useLocalStorage } from "@vlayer/extension-hooks";
import { WebProverSessionConfig } from "../web-proof-commons";

export const useProvingSessionConfig = () => {
  return useLocalStorage<WebProverSessionConfig | undefined>(
    "webProverSessionConfig",
    undefined,
  );
};
