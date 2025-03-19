import { test, expect } from "@playwright/test";
import { stepsMeta, StepKind } from "../src/app/router/types";

test("renders welcome page", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByText(stepsMeta[StepKind.welcome].title)).toBeVisible();
  await expect(
    page.getByText(stepsMeta[StepKind.welcome].description),
  ).toBeVisible();
});
