import browser from "webextension-polyfill";

export class Store<T extends Record<string, unknown>> {
  constructor(private storage: browser.Storage.StorageArea) {}

  async set<S extends keyof T>(key: S, value: T[S]): Promise<void> {
    return await this.storage.set({
      [key]: value,
    });
  }

  async get<S extends keyof T & string>(key: S): Promise<T[S]> {
    const value = await this.storage.get();
    return value[key] as T[S];
  }
}
