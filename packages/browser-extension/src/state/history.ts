import { Store } from "./store";
import browser from "webextension-polyfill";
import { webProverSessionContextManager } from "./webProverSessionContext";

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

  async getUrls(): Promise<string[]> {
    const config =
      await webProverSessionContextManager.getWebProverSessionConfig();
    return config.steps.map((step: { url: string }) => step.url);
  }

  async updateHistory(item: HistoryItem): Promise<void> {
    let newItem = item;
    let history = (await historyContextManager.store.get("history")) || [];
    const existingItemIndex = history.findIndex((i) => i.url === item.url);

    // Add cookies and headers and mark eventually as ready
    if (existingItemIndex !== -1) {
      const existingItem = history[existingItemIndex];
      newItem = {
        ...existingItem,
        ...item,
        // the item becomes ready once it's updated twice (with headers and cookies)
        // ready: true,
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
