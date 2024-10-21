import { expect, test } from "./fixtures";
import { sidePanel } from "./helpers";
import { StepStatus } from "constants/step";
import { type Page } from "playwright";

const config = {
  startPage: "/login",
  expectUrl: "/target",
  notarizeUrl: "https://www.swapi.dev/api/people/1",
};

test.describe("Full flow of webproof using extension", () => {
  test("Full flow from opening sidepanel to redirection", async ({
    page,
    context,
  }) => {
    await test.step("Web-app should open sidepanel via SDK call", async () => {
      await page.goto("/source");
      const requestProofButton = page
        .locator("body")
        .getByTestId("request-webproof-button");
      await requestProofButton.click();
      const extension = await sidePanel(context);
      expect(extension).toBeDefined();
    });

    await test.step("On 'redirect' click extension should open new browser tab for specified startPage url", async () => {
      const extension = await sidePanel(context);

      if (!extension) {
        throw new Error("No sidepanel");
      }
      const redirectButton = extension.getByTestId("start-page-button");
      const [newPage] = await Promise.all([
        context.waitForEvent("page"),
        redirectButton.click(),
      ]);

      await newPage.waitForURL(config.startPage);
    });

    await test.step("Side panel UI should indicate that startPage step is completed", async () => {
      const extension = await sidePanel(context);
      const startPageStep = extension.getByTestId("step-startPage");
      const status = await startPageStep.getAttribute("data-status");
      expect(status).toEqual(StepStatus.Completed);
    });

    await test.step("Side panel UI should indicate that  expectUrl step is completed after redirection", async () => {
      const loginPage = context.pages().find((page) => {
        return page.url().includes("login");
      }) as Page;
      const loginButton = loginPage.getByTestId("login-button");
      await loginButton.click();
      await loginPage.waitForURL(config.expectUrl);
      const extension = await sidePanel(context);
      const startPageStep = extension.getByTestId("step-expectUrl");
      const status = await startPageStep.getAttribute("data-status");
      expect(status).toEqual(StepStatus.Completed);
    });

    await test.step("Prove button should appear after request to external api", async () => {
      const extension = await sidePanel(context);
      const proveButton = extension.getByTestId("prove-button");
      expect(proveButton).toBeDefined();
    });
  });
});
