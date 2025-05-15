import { BrowserContext, expect, Page } from "@playwright/test";
import { Webpage } from "./webpage";
import { waitForSidePanelOpened } from "../helpers";

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
    const newPage = await this.context.waitForEvent("page");
    return new Webpage(newPage, this.context);
  }

  async generateWebProof() {
    const button = this.page.getByRole("button", { name: "Generate proof" });
    await button.click();
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

  async expectErrorToBeDisplayed(expectedErrorMessage: string) {
    await this.page.getByTestId("error-message").waitFor();
    const errorMessage = this.page.getByTestId("error-message");
    await expect(errorMessage).toHaveText(expectedErrorMessage);
  }

  async expectRedirectButtonToBeVisible() {
    const redirectButton = this.page.getByTestId("redirect-button");
    await expect(redirectButton).toBeVisible();
  }

  async expectGenerateProofButtonToBeVisible() {
    const generateProofButton = this.page.getByRole("button", {
      name: "Generate proof",
    });
    await expect(generateProofButton).toBeVisible();
  }

  async expectGenerateProofButtonToBeHidden() {
    const generateProofButton = this.page.getByRole("button", {
      name: "Generate proof",
    });
    await expect(generateProofButton).toBeHidden();
  }

  async expectStepToBeCompleted(stepName: string, stepIndex = 0) {
    const step = this.page.getByTestId(`step-${stepName}`).nth(stepIndex);
    await expect(step).toHaveAttribute("data-status", "completed");
  }

  async expectCountDown() {
    const countdown = this.page.getByText(/You will be redirected back in/i);
    await expect(countdown).toBeVisible();
  }

  async expectCountDownToBeHidden() {
    const countdown = this.page.getByText(/You will be redirected back in/i);
    await expect(countdown).toBeHidden();
  }
}

export const waitForExtension = async (context: BrowserContext) => {
  const extensionPage = await waitForSidePanelOpened(context);
  const extension = new Extension(extensionPage, context);
  return extension;
};
