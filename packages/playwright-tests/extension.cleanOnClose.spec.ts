import { test } from "./config";
import { waitForExtension } from "./pom/extension";
import { Webpage } from "./pom/webpage";
test("Cleanup of storage on extension close", async ({ page, context }) => {
  await test.step("Web-app should open sidepanel via SDK call", async () => {
    await page.goto("/dapp");
    const webpage = new Webpage(page,context);
    await webpage.clickButton("Request proof of beeing a wizard");
    let extension = await waitForExtension(context);
    const newPage = await extension.redirect();
    await newPage.waitForURL("/login");
    await newPage.clickButton("Login");
    await newPage.waitForURL("/dashboard");
    await webpage.closeExtension();
    await webpage.openExtension();
    extension = await waitForExtension(context);
    await extension.startPageStepShouldBeCompleted();
    await extension.expectUrlStepShouldBeCompleted();
    await webpage.finishZkProof();
    await webpage.closeExtension();
    await webpage.openExtension();
    extension = await waitForExtension(context);
    await extension.expectSessionStorageToBeCleaned();
  });
});
