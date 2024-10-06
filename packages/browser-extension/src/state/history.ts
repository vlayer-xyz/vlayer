import { Store } from "./store";
import browser from "webextension-polyfill";
import { webProverSessionContextManager } from "./webProverSessionContext";
import * as R from "ramda";

export type HistoryItem = {
  url: string;
  headers?: browser.WebRequest.HttpHeadersItemType[];
  cookies?: browser.Cookies.Cookie[];
  ready?: boolean;
};

export type History = {
  currentUrl: string;
  history: HistoryItem[];
};

export class HistoryContextManager {
  static #instance: HistoryContextManager;
  store: Store<History>;

  constructor() {
    this.store = new Store<History>(browser.storage.local);
  }

  public static get instance(): HistoryContextManager {
    if (!HistoryContextManager.#instance) {
      HistoryContextManager.#instance = new HistoryContextManager();
    }
    return HistoryContextManager.#instance;
  }

  getUrls = async () => {
    const config =
      await webProverSessionContextManager.getWebProverSessionConfig();
    //@ts-expect-error this is error till we refactor common types
    return config?.steps?.map((step: { url: string }) => step.url) ?? [];
  };

  async updateHistory(item: HistoryItem): Promise<void> {
    let newItem = item;
    let history = (await historyContextManager.store.get("history")) || [];
    const existingItemIndex = history.findIndex((i) => i.url === item.url);

    // Add cookies and headers and mark eventually as ready
    if (existingItemIndex !== -1) {
      const existingItem = history[existingItemIndex];
      newItem = {
        url: item.url,
        headers: R.uniqBy(
          (i) => i?.name,
          [...(existingItem.headers || []), ...(item.headers || [])],
        ),
        cookies: R.uniqBy(
          (i) => i.name,
          [...(existingItem.cookies || []), ...(item.cookies || [])],
        ),
        ready: item.ready ?? existingItem.ready ?? false,
      };
      history = history.map((historyItem, index) => {
        return index === existingItemIndex ? newItem : historyItem;
      });
    } else {
      history = [...history, newItem];
    }
    return historyContextManager.store.set("history", history);
  }
}

export const historyContextManager = HistoryContextManager.instance;
