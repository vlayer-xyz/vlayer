import { LOADING, useLocalStorage } from "@vlayer/extension-hooks";
import { HistoryItem } from "../state/history";
export const useBrowsingHistory = () => {
  const [history] = useLocalStorage<HistoryItem[]>("browsingHistory", []);
  return [history === LOADING ? [] : history];
};
