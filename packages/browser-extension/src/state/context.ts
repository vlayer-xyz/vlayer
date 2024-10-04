import { Store } from "./store";
import browser from "webextension-polyfill";

type HistoryItem = {
  url: string;
  headers?: browser.WebRequest.HttpHeadersItemType[];
  cookies?: browser.Cookies.Cookie[];
  ready?: boolean;
};

type Context = {
  currentUrl: string;
  history: HistoryItem[];
};

export class ContextManager {
  static #instance: ContextManager;
  store: Store<Context>;

  constructor() {
    this.store = new Store<Context>(browser.storage.local);
  }

  public static get instance(): ContextManager {
    if (!ContextManager.#instance) {
      ContextManager.#instance = new ContextManager();
    }
    return ContextManager.#instance;
  }

  getUrls = () => {
    //this should return list of urls that should be recorded based on passed setup
    return ["x.com"];
  };

  async updateHistory(item: HistoryItem): Promise<void> {
    let newItem = item;
    let history = await contextManager.store.get("history");
    const existingItemIndex = history.findIndex((i) => i.url === item.url);

    // Add cookies and headers and mark eventually as ready
    if (existingItemIndex) {
      const existingItem = history[existingItemIndex];
      newItem = {
        url: item.url,
        headers: [...(existingItem.headers || []), ...(item.headers || [])],
        cookies: [...(existingItem.cookies || []), ...(item.cookies || [])],
        ready: item.ready ?? existingItem.ready ?? false,
      };
      history = history.map((historyItem, index) => {
        return index === existingItemIndex ? newItem : historyItem;
      });
    } else {
      history = [...history, newItem];
    }
    return contextManager.store.set("history", history);
  }
}

export const contextManager = ContextManager.instance;
