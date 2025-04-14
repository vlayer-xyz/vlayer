import { test, expect } from "@playwright/test";
import { readFileSync } from "fs";
import { stepsMeta, StepKind } from "../src/app/router/types";
import { useMockWallet } from "./mockWallet";

const testEmlFile = readFileSync("../testdata/valid_vlayer.eml");

test.beforeEach(async ({ page }) => {
  await useMockWallet(page);
});

test("Simple Email Proof happy path", async ({ page }) => {
  await test.step("Navigate to welcome page", async () => {
    await page.goto("/");
    await expect(
      page.getByText(stepsMeta[StepKind.welcome].title),
    ).toBeVisible();
    await expect(
      page.getByText(stepsMeta[StepKind.welcome].description),
    ).toBeVisible();
  });

  await test.step("Click on start button navigates to form with email details", async () => {
    await page.getByTestId("start-page-button").click();
    await expect(page).toHaveURL(stepsMeta[StepKind.sendEmail].path);
    await expect(
      page.getByRole("heading", { name: stepsMeta[StepKind.sendEmail].title }),
    ).toBeVisible();

    await page.route(/email-example-inbox/, (route) => {
      return route.fulfill({
        status: 200,
        contentType: "message/rfc822",
        body: testEmlFile,
      });
    });
  });

  await test.step("Click on next button naviages to mint page", async () => {
    await page.click("#nextButton");
    await expect(page).toHaveURL(stepsMeta[StepKind.mintNft].path);
    await expect(
      page.getByRole("heading", {
        name: stepsMeta[StepKind.mintNft].title,
      }),
    ).toBeVisible();
    await expect(
      page.getByText(stepsMeta[StepKind.mintNft].description),
    ).toBeVisible();
  });

  await test.step("Click on mint button proves, verifies and go to success page", async () => {
    await page.click("#nextButton");
    await expect(page.getByText("Waiting for proof...")).toBeVisible({
      timeout: 30000,
    });
    await expect(page).toHaveURL(/success/, {
      timeout: 60000, // wait for proof
    });
    await expect(
      page.getByRole("heading", {
        name: stepsMeta[StepKind.success].title,
      }),
    ).toBeVisible();
  });
});
