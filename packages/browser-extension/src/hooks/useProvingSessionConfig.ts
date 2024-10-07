import { useLocalStorage } from "@vlayer/extension-hooks";

export const useProvingSessionConfig = () => {
  return useLocalStorage<{
    logoUrl: string;
    notaryUrl: string;
    wsProxyUrl: string;
    steps: {
      label: string;
      step: string;
      url: string;
    }[];
    // REFACTOR : use proper type when commons will be ready
    // eslint-disable-next-line
  }>("webProverSessionConfig", {} as any);
};
