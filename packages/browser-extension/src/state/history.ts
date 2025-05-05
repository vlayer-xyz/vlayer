import { Store } from "./store";
import browser from "webextension-polyfill";
import { webProverSessionContextManager } from "./webProverSessionContext";
import { HTTPMethod } from "lib/HttpMethods";
import { provingSessionStorageConfig } from "./config";

export type BrowsingHistoryItem = {
  url: string;
  headers?: browser.WebRequest.HttpHeadersItemType[];
  cookies?: browser.Cookies.Cookie[];
  ready?: boolean;
  tabId?: number;
  method: HTTPMethod;
  body?: string;
};

export type History = {
  currentUrl: string;
  browsingHistory: BrowsingHistoryItem[];
};

export class HistoryContextManager {
  static #instance: HistoryContextManager;
  store: Store<History>;
  private updateLock: Promise<void> = Promise.resolve();

  constructor() {
    this.store = new Store<History>(provingSessionStorageConfig.storage);
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
    return config?.steps.map((step: { url: string }) => step.url) || [];
  }

  async updateHistory(item: BrowsingHistoryItem): Promise<void> {
    // Prevent concurrent updates to the history
    this.updateLock = this.updateLock.then(async () => {
      let newItem = item;
      let history =
        (await historyContextManager.store.get(
          provingSessionStorageConfig.storageKeys.browsingHistory,
        )) || [];
      const existingItemIndex = history.findIndex(
        (i) => i.url === item.url && i.method === item.method,
      );
      // Add cookies and headers and mark eventually as ready
      if (existingItemIndex !== -1) {
        const existingItem = history[existingItemIndex];
        newItem = {
          ...existingItem,
          ...item,
          // the item becomes ready once it's updated twice (with headers and cookies)
          ready: true,
        };
        history = history.map((historyItem, index) => {
          return index === existingItemIndex ? newItem : historyItem;
        });
      } else {
        history = [...history, newItem];
      }
      await historyContextManager.store.set(
        provingSessionStorageConfig.storageKeys.browsingHistory,
        history,
      );
    });
    return this.updateLock;
  }
}

export const historyContextManager = HistoryContextManager.instance;
