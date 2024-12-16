import { expect, test } from "./config";
import { sidePanel } from "./helpers";

const config = {
  startPage: "/login",
  expectUrl: "/target",
  notarizeUrl: "https://swapi.dev/api/people/1",
};

const VLAYER_SERVER_URL = "http://127.0.0.1:3000";

test.describe("Full flow of webproof using extension", () => {
  test("Full flow from opening sidepanel to redirection", async ({
    page,
    context,
  }) => {
    await test.step("Web-app should open sidepanel via SDK call", async () => {
      await page.goto("/source-new-way");
      const requestProofButton = page
        .locator("body")
        .getByTestId("request-webproof-button");

      await requestProofButton.click();
      const extension = await sidePanel(context);
      expect(extension).toBeDefined();
    });

    await test.step("Extension should stay ok after clicking request button multiple times", async () => {
      await page.goto("/source-new-way");
      const requestProofButton = page
        .locator("body")
        .getByTestId("request-webproof-button");
      await requestProofButton.click();
      await requestProofButton.click();
      await requestProofButton.click();

      const extension = await sidePanel(context);
      expect(extension).toBeDefined();
      const redirectButton = extension.getByTestId("start-page-button");
      await expect(redirectButton).toBeVisible();
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
      expect(status).toEqual("completed");
    });

    await test.step("Side panel UI should indicate that expectUrl step is completed after redirection", async () => {
      const loginPage = context.pages().find((page) => {
        return page.url().includes("login");
      });
      if (!loginPage) {
        throw new Error("No login page");
      }
      const loginButton = loginPage.getByTestId("login-button");
      await loginButton.click();
      await loginPage.waitForURL(config.expectUrl);
      const extension = await sidePanel(context);
      const startPageStep = extension.getByTestId("step-expectUrl");
      const status = await startPageStep.getAttribute("data-status");
      expect(status).toEqual("completed");
    });

    await test.step("Prove button should appear after request to external api", async () => {
      const extension = await sidePanel(context);
      const proveButton = extension.getByTestId("prove-button");
      await expect(proveButton).toHaveText("Generate proof");
    });

    await test.step("Click button should generate webproof", async () => {
      const extension = await sidePanel(context);
      const proveButton = extension.getByTestId("prove-button");
      await proveButton.click();
      await page.reload();
      await page.waitForSelector('h1[data-testid="has-webproof"]');
    });

    await test.step("Zk prove button should appear after receiving webProof", async () => {
      const proveButton = page.locator("body").getByTestId("zk-prove-button");
      await expect(proveButton).toBeVisible();
    });

    await test.step("Proving request has succeeded", async () => {
      const proveButton = page.locator("body").getByTestId("zk-prove-button");
      await proveButton.click();

      const response = await page.waitForResponse(VLAYER_SERVER_URL);
      expect(response.ok()).toBeTruthy();

      const response_json = (await response.json()) as object;
      expect(response_json).toHaveProperty("result");

      const hash = (response_json as { result: string }).result;
      expect(hash).toBeValidHash();
    });
  });

  test("Full flow from opening sidepanel to redirection with legacy communication", async ({
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
      expect(status).toEqual("completed");
    });

    await test.step("Side panel UI should indicate that  expectUrl step is completed after redirection", async () => {
      const loginPage = context.pages().find((page) => {
        return page.url().includes("login");
      });
      if (!loginPage) {
        throw new Error("No login page");
      }
      const loginButton = loginPage.getByTestId("login-button");
      await loginButton.click();
      await loginPage.waitForURL(config.expectUrl);
      const extension = await sidePanel(context);
      const startPageStep = extension.getByTestId("step-expectUrl");
      const status = await startPageStep.getAttribute("data-status");
      expect(status).toEqual("completed");
    });

    await test.step("Prove button should appear after request to external api", async () => {
      const extension = await sidePanel(context);
      const proveButton = extension.getByTestId("prove-button");
      expect(proveButton).toBeDefined();
    });

    await test.step("Click button should generate valid hash for zk proof", async () => {
      const extension = await sidePanel(context);
      const proveButton = extension.getByTestId("prove-button");
      await proveButton.click();
      await page.waitForSelector('h1[data-testid="has-webproof"]');

      const response = await page.waitForResponse(VLAYER_SERVER_URL);
      expect(response.ok()).toBeTruthy();

      const response_json = (await response.json()) as object;
      expect(response_json).toHaveProperty("result");

      const hash = (response_json as { result: string }).result;
      expect(hash).toBeValidHash();
    });
  });
});
