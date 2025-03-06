import { Store } from "./store";
import browser from "webextension-polyfill";
import { ZkProvingStatus } from "../web-proof-commons";
import { provingSessionStorageConfig } from "./config";

//this belongs to the service worker so it does not use react under the hood

export class ZkProvingStatusStore extends Store<{
  zkProvingStatus: ZkProvingStatus;
}> {
  static #instance: ZkProvingStatusStore;

  private constructor(storage: browser.Storage.StorageArea) {
    super(storage);
  }

  public static get instance(): ZkProvingStatusStore {
    if (!this.#instance) {
      this.#instance = new ZkProvingStatusStore(
        provingSessionStorageConfig.storage,
      );
    }
    return this.#instance;
  }

  async setProvingStatus({
    status,
  }: {
    status: ZkProvingStatus;
  }): Promise<void> {
    await this.set(
      provingSessionStorageConfig.storageKeys.zkProvingStatus,
      status,
    );
  }

  async getProvingStatus(): Promise<ZkProvingStatus> {
    return await this.get(
      provingSessionStorageConfig.storageKeys.zkProvingStatus,
    );
  }
}

export const zkProvingStatusStore = ZkProvingStatusStore.instance;
