import { useSessionStorage } from "@vlayer/extension-hooks";
import { WebProverSessionConfig } from "../web-proof-commons";

export const useProvingSessionConfig = () => {
  const [config] = useSessionStorage<WebProverSessionConfig>(
    "webProverSessionConfig",
  );
  return [config];
};
