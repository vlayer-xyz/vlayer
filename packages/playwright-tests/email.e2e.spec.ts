import { expect, test } from "./config";
import path from "path";

import { fileURLToPath } from "url";
import { emailUrl } from "./urls";
const VLAYER_SERVER_URL = "http://127.0.0.1:3000";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

test("Success email proof flow", async ({ page }) => {
  await page.goto(emailUrl);
  await expect(page.locator("h1")).toHaveText("Email");
  const fileChooserPromise = page.waitForEvent("filechooser");
  await page.locator('input[name="file"]').click();
  const fileChooser = await fileChooserPromise;
  await fileChooser.setFiles(__dirname + "/fixtures/verify_vlayer.eml");

  const response = await page.waitForResponse(VLAYER_SERVER_URL);
  expect(response.ok()).toBeTruthy();

  const response_json = (await response.json()) as object;
  expect(response_json).toHaveProperty("result");

  const hash = (response_json as { result: string }).result;
  expect(hash).toBeValidHash();
});
