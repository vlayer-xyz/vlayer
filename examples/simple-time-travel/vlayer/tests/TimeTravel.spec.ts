import { test, expect } from "@playwright/test";
import { stepsMeta, StepKind } from "../src/app/router/types";
import { useMockWallet } from "./mockWallet";

test.beforeEach(async ({ page }) => {
  await useMockWallet(page);
});

test("Simple time travel flow", async ({ page }) => {
  await test.step("renders welcome page", async () => {
    await page.goto("/");
    await expect(
      page.getByText(stepsMeta[StepKind.welcome].title),
    ).toBeVisible();
    await expect(
      page.getByText(stepsMeta[StepKind.welcome].description),
    ).toBeVisible();
  });

  await test.step("generates proof and navigates to show  proven balance", async () => {
    await page.click("#nextButton");
    await expect(page).toHaveURL(/show-balance/, {
      timeout: 30000,
    });
    await expect(page.getByText("Average balance")).toBeVisible();
  });

  await test.step("verifies proof and navigates to success", async () => {
    await page.click("#nextButton");
    await expect(page).toHaveURL(/success/, {
      timeout: 30000,
    });
    await expect(
      page.getByRole("heading", {
        name: stepsMeta[StepKind.success].description,
      }),
    ).toBeVisible();
  });
});
