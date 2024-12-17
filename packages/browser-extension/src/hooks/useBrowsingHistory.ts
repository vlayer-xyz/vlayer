import { LOADING, useSessionStorage } from "@vlayer/extension-hooks";
import { BrowsingHistoryItem } from "../state/history";
export const useBrowsingHistory = () => {
  const [history] = useSessionStorage<BrowsingHistoryItem[]>(
    "browsingHistory",
    [],
  );
  return [history === LOADING ? [] : history];
};
