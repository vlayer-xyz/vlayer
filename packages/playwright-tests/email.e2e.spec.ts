import { expect, test } from "@playwright/test";

test("email", async ({ page }) => {
  await page.goto("/email");
  await expect(page.locator("h1")).toHaveText("Email");
});

