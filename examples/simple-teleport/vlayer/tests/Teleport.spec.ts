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

  await test.step("renders show balance page", async () => {
    await page.getByText("Show cross-chain balance").click();
    await expect(
      page.getByText(stepsMeta[StepKind.showBalance].title),
    ).toBeVisible();
    await expect(
      page.getByText(stepsMeta[StepKind.showBalance].description),
    ).toBeVisible();
  });

  await test.step("renders confirm mint page", async () => {
    await page.getByText("Generate Proof NFT").click();
    await expect(
      page.getByText(stepsMeta[StepKind.confirmMint].title),
    ).toBeVisible();
  });

  await test.step("renders success page", async () => {
    await page.getByText("Mint token").click();
    await expect(
      page.getByText(stepsMeta[StepKind.success].title),
    ).toBeVisible();
  });
});
