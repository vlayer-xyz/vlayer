import { type BrowserContext, type Page } from "@playwright/test";

export const pageByUrlRegex = (
  context: BrowserContext,
  regex: RegExp,
): Page | undefined => {
  return context.pages().find((page) => regex.test(page.url()));
};

export const sidePanel = async (context: BrowserContext) => {
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

export const sidePanelClosed = async (context: BrowserContext) => {
  let [background] = context.serviceWorkers();
  if (!background) {
    background = await context.waitForEvent("serviceworker");
  }
  const extensionId = background.url().split("/")[2];
  const regex = new RegExp(`.*${extensionId}.*`);
  let sidepanel = pageByUrlRegex(context, regex);
  while (sidepanel) {
    //
    await new Promise((resolve) => setTimeout(resolve, 1000));
    sidepanel = pageByUrlRegex(context, regex);
  }
  return {
    closed: true,
  };
};
