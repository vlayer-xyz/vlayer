import { test, expect } from "@playwright/test";

test("verifies valid eml file", async ({ page }) => {
  await page.goto("/");
  await page
    .getByLabel("EML File Upload")
    .setInputFiles("./testdata/verify_vlayer.eml");
  await page.getByRole("button", { name: "Connect & Claim NFT" }).click();
  await expect(page.getByText("Verified")).toBeVisible({ timeout: 60000 });
});

test("raises error for invalid eml file", async ({ page }) => {
  await page.goto("/");
  await page
    .getByLabel("EML File Upload")
    .setInputFiles("./testdata/incorrect_vlayer.eml");
  await page.getByRole("button", { name: "Connect & Claim NFT" }).click();
  await expect(page.getByText("Error:")).toBeVisible();
});
