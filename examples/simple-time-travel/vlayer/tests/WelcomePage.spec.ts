import { test, expect } from "@playwright/test";
import { stepsMeta, StepKind } from "../src/app/router/types";

test("renders welcome page", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByText(stepsMeta[StepKind.welcome].title)).toBeVisible();
  await expect(
    page.getByText(stepsMeta[StepKind.welcome].description),
  ).toBeVisible();
});

test("navigates to show balance page", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("button").click();
  await expect(
    page.getByText(stepsMeta[StepKind.showBalance].title),
  ).toBeVisible();
  await expect(
    page.getByText(stepsMeta[StepKind.showBalance].description),
  ).toBeVisible();
});
