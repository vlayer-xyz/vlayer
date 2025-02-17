import { test, expect } from "@playwright/test";
import { steps } from "../src/app/router/steps";

test("navigates from / to wallet connection page", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByText(steps[0].title)).toBeVisible();
  await expect(page.getByText(steps[0].description)).toBeVisible();
  await page.getByTestId("start-page-button").click();
  await expect(page).toHaveURL("/connect-wallet");
});
