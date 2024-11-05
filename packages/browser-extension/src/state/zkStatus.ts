import { Store } from "./store";
import browser from "webextension-polyfill";
import { ZkProvingStatus } from "../web-proof-commons";

export class ZkProvingStatusManager extends Store<{
  zkProvingStatus: ZkProvingStatus;
}> {
  static #instance: ZkProvingStatusManager;

  private constructor(storage: browser.Storage.StorageArea) {
    super(storage);
  }

  public static get instance(): ZkProvingStatusManager {
    if (!this.#instance) {
      this.#instance = new ZkProvingStatusManager(browser.storage.local);
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

export const zkProvingStatusManager = ZkProvingStatusManager.instance;
