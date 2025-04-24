import { test, expect } from "@playwright/test";
import { stepsMeta, StepKind } from "../src/app/router/types";
import { useMockWallet } from "./mockWallet";

test.beforeEach(async ({ page }) => {
  await useMockWallet(page);
});

test("Simple teleport flow", async ({ page }) => {
  await test.step("renders welcome page", async () => {
    await page.goto("/");
    await expect(
      page.getByText(stepsMeta[StepKind.welcome].title),
    ).toBeVisible();
    await expect(
      page.getByText(stepsMeta[StepKind.welcome].description),
    ).toBeVisible();
  });
});
