import { test, expect } from "@playwright/test";
import { stepsMeta, STEP_KIND } from "../src/app/router/types";

test("navigates from / to wallet connection page", async ({ page }) => {
  await page.goto("/");
  await expect(
    page.getByText(stepsMeta[STEP_KIND.WELCOME].title),
  ).toBeVisible();
  await expect(
    page.getByText(stepsMeta[STEP_KIND.WELCOME].description),
  ).toBeVisible();
  await page.getByTestId("start-page-button").click();
  await expect(page).toHaveURL(stepsMeta[STEP_KIND.CONNECT_WALLET].path);
});
