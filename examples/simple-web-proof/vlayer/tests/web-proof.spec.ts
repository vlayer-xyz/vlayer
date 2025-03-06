import { expect } from "playwright/test";
import { test } from "./config";
import { sidePanel } from "./helpers";

test("web proof flow", async ({ page, context }) => {
  // To obtain this token, log in to x.com and copy the value of the `auth_token` cookie.
  const authToken = process.env.PLAYWRIGHT_TEST_X_COM_AUTH_TOKEN;

  if (!authToken) {
    throw new Error(
      "Missing required environment variable: PLAYWRIGHT_TEST_X_COM_AUTH_TOKEN",
    );
  }

  await context.addCookies([
    {
      name: "auth_token",
      value: authToken,
      path: "/",
      domain: ".x.com",
    },
  ]);

  await test.step("Click Start", async () => {
    await page.goto("/");

    await expect(page.getByTestId("start-page-button")).toBeVisible();
    await page.getByTestId("start-page-button").click();

    await expect(page).toHaveURL("/connect-wallet");
  });

  await test.step("Click Start Proving", async () => {
    await expect(page.getByText("Start Proving")).toBeVisible();
    await page.getByText("Start Proving").click();

    await expect(page).toHaveURL("/start-proving");
  });

  await test.step("Open extension", async () => {
    await expect(page.getByText("Open Extension")).toBeVisible();
    await page.getByText("Open Extension").click();

    const extension = await sidePanel(context);
    expect(extension).toBeDefined();
  });

  await test.step("Redirect to x.com", async () => {
    const extension = await sidePanel(context);
    expect(extension).toBeDefined();

    const redirectButton = extension.getByTestId("start-page-button");
    const [newPage] = await Promise.all([
      context.waitForEvent("page"),
      redirectButton.click(),
    ]);

    await expect(newPage).toHaveURL("https://x.com/home");
  });

  await test.step("Click generate proof", async () => {
    const extension = await sidePanel(context);
    const proveButton = extension.getByRole("button", {
      name: "Generate proof",
    });
    await proveButton.click();
    await expect(page.getByText("Start Minting")).toBeVisible({
      timeout: 1200_000,
    });
  });
});
