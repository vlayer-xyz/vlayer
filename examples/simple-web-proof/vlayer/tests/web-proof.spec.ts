import { test, expect } from "@playwright/test";

test.beforeAll(({ playwright }) => {
  console.log("Resolved Playwright Config:", JSON.stringify(playwright._initializer, null, 2));
});

test("displays button", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("start-page-button")).toBeVisible({
    timeout: 60000,
  });
});
