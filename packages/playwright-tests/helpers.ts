import { type BrowserContext, type Page } from "@playwright/test";

export const pageByUrlRegex = (
  context: BrowserContext,
  regex: RegExp,
): Page | undefined => {
  return context.pages().find((page) => regex.test(page.url()));
};

export const waitForSidePanelOpened = async (context: BrowserContext) => {
  let [background] = context.serviceWorkers();
  if (!background) {
    background = await context.waitForEvent("serviceworker");
  }
  const extensionId = background.url().split("/")[2];
  const regex = new RegExp(`.*${extensionId}.*`);
  let sidepanel = pageByUrlRegex(context, regex);
  while (!sidepanel) {
    await context.waitForEvent("page");
    sidepanel = pageByUrlRegex(context, regex);
  }
  return sidepanel;
};

export const waitForSidePanelClosed = async (context: BrowserContext) => {
  let [background] = context.serviceWorkers();
  if (!background) {
    background = await context.waitForEvent("serviceworker");
  }
  const extensionId = background.url().split("/")[2];
  const regex = new RegExp(`.*${extensionId}.*`);
  let sidepanel = pageByUrlRegex(context, regex);
  while (sidepanel) {
    // there is no event for sidepanel close, so we need to
    // have such a sleep
    await new Promise((resolve) => setTimeout(resolve, 100));
    sidepanel = pageByUrlRegex(context, regex);
  }
  return {
    closed: true,
  };
};
