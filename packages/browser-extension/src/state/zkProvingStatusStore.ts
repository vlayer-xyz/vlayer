import { Store } from "./store";
import browser from "webextension-polyfill";
import { ZkProvingStatus } from "../web-proof-commons";

export class ZkProvingStatusStore extends Store<{
  zkProvingStatus: ZkProvingStatus;
}> {
  static #instance: ZkProvingStatusStore;

  private constructor(storage: browser.Storage.StorageArea) {
    super(storage);
  }

  public static get instance(): ZkProvingStatusStore {
    if (!this.#instance) {
      this.#instance = new ZkProvingStatusStore(browser.storage.session);
    }
    return this.#instance;
  }

  async setProvingStatus({
    status,
  }: {
    status: ZkProvingStatus;
  }): Promise<void> {
    await this.set("zkProvingStatus", status);
  }

  async getProvingStatus(): Promise<ZkProvingStatus> {
    return await this.get("zkProvingStatus");
  }
}

export const zkProvingStatusStore = ZkProvingStatusStore.instance;
