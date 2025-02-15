import { test, expect } from "./config";
import { sidePanel } from "./helpers";

export const config = {
  loginUrl: "/login",
  profileUrl: "/profile",
  profileFailedAuthUrl: "/profile-failed-auth",
  dashboardUrl: "/dashboard",
};

test("Full flow from opening sidepanel to redirection for /dapp-put", async ({
  page,
  context,
}) => {
  await test.step("Web-app should open sidepanel via SDK call", async () => {
    await page.goto("/dapp-put");
    const requestProofButton = page
      .locator("body")
      .getByTestId("request-webproof-button");
    await requestProofButton.click();
    const extension = await sidePanel(context);
    expect(extension).toBeDefined();
  });

  await test.step("Extension should stay ok after clicking request button multiple times", async () => {
    await page.goto("/dapp-put");
    const requestProofButton = page
      .locator("body")
      .getByTestId("request-webproof-button");
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

    await newPage.waitForURL(config.loginUrl);
  });

  await test.step("Side panel UI should indicate that startPage step is completed", async () => {
    const extension = await sidePanel(context);
    const startPageStep = extension.getByTestId("step-startPage");
    const status = await startPageStep.getAttribute("data-status");
    expect(status).toEqual("completed");
  });

  await test.step("Side panel UI should indicate that expectUrl step is completed after history.pushState redirect", async () => {
    const startPage = context.pages().find((page) => {
      return page.url().includes(config.loginUrl);
    });
    if (!startPage) {
      throw new Error("No login page");
    }
    const loginButton = startPage.getByTestId("login-button");
    await loginButton.click();
    const extension = await sidePanel(context);
    const startPageStep = extension.getByTestId("step-expectUrl").nth(0);
    const status = await startPageStep.getAttribute("data-status");

    expect(status).toEqual("completed");
  });

  await test.step("Side panel UI should activate prove button after clicking update resource button", async () => {
    const dashboardPage = context.pages().find((page) => {
      return page.url().includes(config.dashboardUrl);
    });
    if (!dashboardPage) {
      throw new Error("No dashboard page");
    }
    const profileButton = dashboardPage.getByTestId("update-resource-button");
    await Promise.all([
      context.waitForEvent(
        "response",
        (response) =>
          response.url().includes("update_resource") &&
          response.status() === 200,
      ),
      profileButton.click(),
    ]);
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
});
