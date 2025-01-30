import { test, expect } from "@playwright/test";

test("displays button", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("start-page-button")).toBeVisible({
    timeout: 60000,
  });
});
