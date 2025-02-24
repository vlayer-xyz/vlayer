import { test, expect } from "@playwright/test";

test("displays button", async ({ page }) => {
  await test.step("Click Start", async () => {
    await page.goto("/");

    await expect(page.getByTestId("start-page-button")).toBeVisible();
    await page.getByTestId("start-page-button").click();

    await expect(page).toHaveURL("/connect-wallet");
  });

  await test.step("Click Start Proving", async () => {
    await expect(page.getByText("Start Proving")).toBeVisible();
    await page.getByText("Start Proving").click();

    await expect(page).toHaveURL("/install-extension");
  });
});
