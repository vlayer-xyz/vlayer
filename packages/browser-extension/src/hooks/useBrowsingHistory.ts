import { LOADING, useLocalStorage } from "@vlayer/extension-hooks";
import { BrowsingHistoryItem } from "../state/history";
export const useBrowsingHistory = () => {
  const [history] = useLocalStorage<BrowsingHistoryItem[]>(
    "browsingHistory",
    [],
  );
  return [history === LOADING ? [] : history];
};
