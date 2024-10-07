import { useLocalStorage } from "@vlayer/extension-hooks";
import { HistoryItem } from "../state/history";
export const useBrowsingHistory = () => {
  return useLocalStorage<HistoryItem[]>("history", []);
};
