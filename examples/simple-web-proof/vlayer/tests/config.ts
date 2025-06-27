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
  context: async ({ }, load) => {
    const pathToExtension = path.join(__dirname, "./tests/browser-extension");

    const context = await chromium.launchPersistentContext(
      "",
      process.env.TEST_MODE === "headed"
        ? {
            headless: false,
            args: [
              `--disable-extensions-except=${pathToExtension}`,
              `--load-extension=${pathToExtension}`,
            ],
          }
        : {
            headless: true,
            args: [
              `--headless=new`,
              `--disable-extensions-except=${pathToExtension}`,
              `--load-extension=${pathToExtension}`,
            ],
          },
    );

    await load(context);
    await context.close();
  },
});
