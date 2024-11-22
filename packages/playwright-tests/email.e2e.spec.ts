import { expect, test } from "@playwright/test";
import path from "path";

import { fileURLToPath } from "url";
const VLAYER_SERVER_URL = "http://127.0.0.1:3000";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

test("Success email proof flow", async ({ page }) => {
  await page.goto("/email");
  await expect(page.locator("h1")).toHaveText("Email");
  const fileChooserPromise = page.waitForEvent("filechooser");
  await page.locator('input[name="file"]').click();
  const fileChooser = await fileChooserPromise;
  await fileChooser.setFiles(__dirname + "/fixtures/vlayer_welcome.eml");

  const response = await page.waitForResponse(VLAYER_SERVER_URL);
  expect(response.ok()).toBeTruthy();

  const response_json = (await response.json()) as object;
  expect(response_json).toHaveProperty("result.proof");
});
