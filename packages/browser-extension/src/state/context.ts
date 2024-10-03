import { Store } from "./store";
import browser from "webextension-polyfill";

type HistoryItem = {
  url: string;
  headers: {
    [key: string]: string;
  };
};

type Context = {
  currentUrl: string;
  history: HistoryItem[];
};

export class ContextManager extends Store<Context> {
  static #instance: Store<Context>;

  public static get instance(): Store<Context> {
    if (!ContextManager.#instance) {
      ContextManager.#instance = new Store<Context>(browser.storage.local);
    }
    return ContextManager.#instance;
  }

  async pushHistory(item: HistoryItem): Promise<void> {
    const history = await ContextManager.instance.get("history");
    return ContextManager.instance.set("history", [...history, item]);
  }
}
