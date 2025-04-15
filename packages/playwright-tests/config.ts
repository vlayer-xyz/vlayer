import {
  type Page,
  test as base,
  chromium,
  type BrowserContext,
} from "@playwright/test";
import * as path from "path";
const __dirname = path.resolve();
// NOTE: this env make sidepanel accessible vis context.pages()
// https://github.com/microsoft/playwright/blob/8f3353865d8d98e9b40c15497e60d5e2583410b6/packages/playwright-core/src/server/chromium/crBrowser.ts#L169C11-L169C27
process.env.PW_CHROMIUM_ATTACH_TO_OTHER = "1";

export const test = base.extend<{
  context: BrowserContext;
  page: Page;
}>({
  // eslint-disable-next-line
  context: async ({}, load) => {
    const pathToExtension = path.join(__dirname, "browser-extension/dist");

    const context = await chromium.launchPersistentContext(
      "",
      process.env.TEST_MODE === "headed"
        ? {
            headless: false,
            args: [
              `--disable-extensions-except=${pathToExtension}`,
              `--load-extension=${pathToExtension}`,
              "--host-resolver-rules=MAP lotr-api.online 127.0.0.1",
            ],
          }
        : {
            headless: true,
            args: [
              `--headless=new`,
              `--disable-extensions-except=${pathToExtension}`,
              `--load-extension=${pathToExtension}`,
              "--host-resolver-rules=MAP lotr-api.online 127.0.0.1",
            ],
          },
    );

    await load(context);
    await context.close();
  },
});

export const expect = test.expect.extend({
  toBeValidHash: (hash: string) => {
    const PREFIX = "0x";
    const HASH_LENGTH = 32;
    const pass =
      hash.startsWith(PREFIX) &&
      hash.slice(PREFIX.length).length === HASH_LENGTH * 2;
    return {
      pass,
      message: () => `expected ${hash} to be a valid hash`,
    };
  },
});

export const extensionId = "jbchhcgphfokabmfacnkafoeeeppjmpl";
