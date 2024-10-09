import { test, expect } from "./fixtures";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

test.describe("side panel", () => {
  test("happy path", async ({ page, extensionId }) => {
    await page.goto("http://localhost:5174");
    const requestProofButton = await page.locator("body").getByTestId("prove");
    await requestProofButton.click();
    //TODO : find better way to await sidepanel open
    await sleep(1000);
    const extension = page
      .context()
      .pages()
      .find((page) => {
        return page.url().includes(extensionId);
      });

    expect(extension).toBeDefined();
  });
});
