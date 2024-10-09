import { test as base, chromium, type BrowserContext } from "@playwright/test";
import path from "path";

export const test = base.extend<{
  context: BrowserContext;
  extensionId: string;
}>({
  // eslint-disable-next-line
  context: async ({}, use) => {
    const pathToExtension = path.join(import.meta.dirname, "../dist");

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
    await use(context);
    await context.close();
  },
  extensionId: async ({ context }, use) => {
    let [background] = context.serviceWorkers();
    if (!background) background = await context.waitForEvent("serviceworker");
    const extensionId = background.url().split("/")[2];
    await use(extensionId);
  },
});

export const expect = test.expect;
