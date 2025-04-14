import { test } from "./config";
import { waitForExtension } from "./pom/extension";
import { Webpage } from "./pom/webpage";
import { dappUrl, dashboardUrl, loginUrl } from "./urls";

// For now to remove flakiness of tests we need to delay messages
const delayMessage = async () => {
  await new Promise((resolve) => setTimeout(resolve, 1000));
};

test("Cleanup of storage on extension close", async ({ page, context }) => {
  await page.goto(dappUrl);
  const webpage = new Webpage(page, context);
  await webpage.clickButton("Request proof of being a wizard");

  let extension = await waitForExtension(context);

  const newPage = await extension.redirect();
  await newPage.waitForURL(loginUrl);

  await newPage.clickButton("Login");
  await newPage.waitForURL(dashboardUrl);

  // close and reopen
  await webpage.closeExtension();
  await delayMessage();
  await webpage.openExtension();
  extension = await waitForExtension(context);
  // make sure storage is not cleaned as steps are rendered properly
  await extension.startPageStepShouldBeCompleted();
  await extension.expectUrlStepShouldBeCompleted();

  await webpage.finishZkProof();

  // close and reopen after finish flow now session storage is expected to be cleaned
  await webpage.closeExtension();
  await delayMessage();
  await webpage.openExtension();
  extension = await waitForExtension(context);
  await extension.expectSessionStorageToBeCleaned();
});
