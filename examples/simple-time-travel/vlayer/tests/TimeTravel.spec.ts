import { test, expect } from "@playwright/test";
import { stepsMeta, StepKind } from "../src/app/router/types";

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

  await test.step("navigates to show balance page", async () => {
    await page.getByRole("button").click();
    await expect(
      page.getByText(stepsMeta[StepKind.showBalance].title),
    ).toBeVisible();
    await expect(
      page.getByText(stepsMeta[StepKind.showBalance].description),
    ).toBeVisible();
  });
});
