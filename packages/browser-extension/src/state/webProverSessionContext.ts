import { Store } from "./store";
import browser from "webextension-polyfill";
import { WebProverSessionConfig } from "../web-proof-commons";

type WebProverSessionContext = {
  webProverSessionConfig: WebProverSessionConfig;
};

export class WebProverSessionContextManager extends Store<WebProverSessionContext> {
  static #instance: WebProverSessionContextManager;

  private constructor(storage: browser.Storage.StorageArea) {
    super(storage);
  }

  public static get instance(): WebProverSessionContextManager {
    if (!this.#instance) {
      this.#instance = new WebProverSessionContextManager(
        browser.storage.session,
      );
    }
    return this.#instance;
  }

  async setWebProverSessionConfig(
    config: WebProverSessionConfig,
  ): Promise<void> {
    await this.set("webProverSessionConfig", config);
  }

  async getWebProverSessionConfig(): Promise<WebProverSessionConfig> {
    return await this.get("webProverSessionConfig");
  }
}

export const webProverSessionContextManager =
  WebProverSessionContextManager.instance;
