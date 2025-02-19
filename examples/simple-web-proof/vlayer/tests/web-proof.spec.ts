import { test, expect } from "@playwright/test";

test("displays button", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("start-page-button")).toBeVisible({
    timeout: 60000,
  });

  await page.getByTestId("start-page-button").click();
  await expect(page).toHaveURL("/connect-wallet");

  await expect(page.getByTestId("start-proving-button")).toBeVisible();
  await page.getByTestId("start-proving-button").click();

  await expect(page).toHaveURL("/install-extension");
});
