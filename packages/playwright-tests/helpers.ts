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
  const sidepanel = pageByUrlRegex(context, regex);
  if (!sidepanel) {
    await context.waitForEvent("page");
    return pageByUrlRegex(context, regex) as Page;
  } else {
    return sidepanel;
  }
};
