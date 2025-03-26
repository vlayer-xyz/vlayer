import { expect, test } from "./config";
import { sidePanel } from "./helpers";

const config = {
  loginUrl: "/login",
  profileUrl: "/profile",
  profileFailedAuthUrl: "/profile-failed-auth",
  dashboardUrl: "/dashboard",
};

test("Flow from opening sidepanel until 403 from proven endpoint", async ({
  page,
  context,
}) => {
  await test.step("Web-app should open sidepanel via SDK call", async () => {
    await page.goto("/dapp-failed-auth");
    const requestProofButton = page
      .locator("body")
      .getByTestId("request-webproof-button");

    await requestProofButton.click();
    const extension = await sidePanel(context);

    const redirectButton = extension.getByTestId("start-page-button");
    const [newPage] = await Promise.all([
      context.waitForEvent("page"),
      redirectButton.click(),
    ]);

    await newPage.waitForURL(config.loginUrl);

    const startPage = context.pages().find((page) => {
      return page.url().includes(config.loginUrl);
    });
    if (!startPage) {
      throw new Error("No login page");
    }
    const loginButton = startPage.getByRole("button", {
      name: "Login",
    });
    await loginButton.click();

    const dashboardPage = context.pages().find((page) => {
      return page.url().includes(config.dashboardUrl);
    });
    if (!dashboardPage) {
      throw new Error("No dashboard page");
    }
    const profileButton = dashboardPage.getByRole("button", {
      name: "Go to profile failed auth",
    });
    await profileButton.click();
    await dashboardPage.waitForURL(config.profileFailedAuthUrl);

    const proveButton = extension.getByRole("button", {
      name: "Generate proof",
    });
    await expect(proveButton).toHaveText("Generate proof");

    await proveButton.click();

    await extension.waitForSelector('p[data-testid="error-message"]');
    const errorMessage = extension.getByTestId("error-message");
    await expect(errorMessage).toHaveText(
      "Authentication failed. Please restart the process.",
    );
  });
});
