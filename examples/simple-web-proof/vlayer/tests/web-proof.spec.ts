import { test, expect } from "@playwright/test";

test("displays button", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("button", { name: "Start" })).toBeVisible({ 
    timeout: 60000
  });
});
