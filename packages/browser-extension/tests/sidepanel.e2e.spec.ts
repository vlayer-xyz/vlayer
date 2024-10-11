import { expect, test } from "./fixtures";
import { sidePanel } from "./helpers";
test("web-app is able to properly open sidepanel via SDK call ", async ({
  page,
  context,
}) => {
  await page.goto("/");
  const requestProofButton = page
    .locator("body")
    .getByTestId("request-webproof-button");
  await requestProofButton.click();
  const extension = await sidePanel(context);
  if (!extension) {
    throw new Error("No sidepanel ");
  }
  const redirectButton = extension.getByTestId("start-page-button");
  await expect(redirectButton).toHaveText("Redirect");
});
