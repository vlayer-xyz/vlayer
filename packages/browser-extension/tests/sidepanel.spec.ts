import { test, expect } from "./fixtures";

test("side panel opened ", async ({ page, extensionId, context }) => {
  await page.goto("http://localhost:5174");
  const extension = await context.pages().find((page) => {
    return page.url().includes(extensionId);
  });
  console.log(
    context.pages().map((page) => {
      console.log("pa", page.url());
    }),
  );
  if (!extension) {
    throw new Error(`Unable to find ${extensionId}`);
  }
  const requestProofButton = await page.locator("body").getByTestId("prove");
  await requestProofButton.click();
  await expect(
    extension.locator("body").getByTestId("side-panel"),
  ).toBeVisible();
});
