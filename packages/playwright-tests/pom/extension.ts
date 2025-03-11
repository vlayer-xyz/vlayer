import { BrowserContext, expect, Page } from "@playwright/test";
import { Webpage } from "./webpage";
import { sidePanel } from "../helpers";

export class Extension {
  constructor(
    private readonly page: Page,
    private readonly context: BrowserContext,
  ) {
    this.page = page;
    this.context = context;
  }

  getRedirectButton() {
    return this.page.getByRole("button", { name: "Redirect" });
  }

  async redirect() {
    const button = this.getRedirectButton();
    const [newPage] = await Promise.all([
      this.context.waitForEvent("page"),
      button.click(),
    ]);
    return new Webpage(newPage);
  }
  async startPageStepShouldBeCompleted() {
    const startPageStep = this.page.getByTestId("step-startPage");
    const status = await startPageStep.getAttribute("data-status");
    expect(status).toEqual("completed");
  }

  async expectUrlStepShouldBeCompleted() {
    const expectUrlStep = this.page.getByTestId("step-expectUrl").nth(0);
    const status = await expectUrlStep.getAttribute("data-status");
    expect(status).toEqual("completed");
  }
  async expectSessionStorageToBeCleaned() {
    const sessionStorage = await this.page.evaluate(() =>
      chrome.storage.session.get(),
    );
    expect(sessionStorage).toEqual({});
  }
}

export const waitForExtension = async (context: BrowserContext) => {
  const extensionPage = await sidePanel(context);
  const extension = new Extension(extensionPage, context);
  return extension;
};
