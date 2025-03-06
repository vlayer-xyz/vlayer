import { Store } from "./store";
import browser from "webextension-polyfill";
import { WebProverSessionConfig } from "../web-proof-commons";
import { provingSessionStorageConfig } from "./config";

type WebProverSessionContext = {
  webProverSessionConfig: WebProverSessionConfig;
};

//this belongs to the service worker so it does not use react under the hood

export class WebProverSessionContextManager extends Store<WebProverSessionContext> {
  static #instance: WebProverSessionContextManager;

  private constructor(storage: browser.Storage.StorageArea) {
    super(storage);
  }

  public static get instance(): WebProverSessionContextManager {
    if (!this.#instance) {
      this.#instance = new WebProverSessionContextManager(
        provingSessionStorageConfig.storage,
      );
    }
    return this.#instance;
  }

  async setWebProverSessionConfig(
    config: WebProverSessionConfig,
  ): Promise<void> {
    await this.set(
      provingSessionStorageConfig.storageKeys.webProverSessionConfig,
      config,
    );
  }

  async getWebProverSessionConfig(): Promise<WebProverSessionConfig> {
    return await this.get(
      provingSessionStorageConfig.storageKeys.webProverSessionConfig,
    );
  }
}

export const webProverSessionContextManager =
  WebProverSessionContextManager.instance;
