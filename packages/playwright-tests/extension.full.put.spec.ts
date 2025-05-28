import { test, expect } from "./config";
import { waitForExtension } from "./pom/extension";
import { Webpage } from "./pom/webpage";
import { loginUrl, dashboardUrl, dappPutUrl } from "./urls";

test("Full flow from opening sidepanel to redirection for /dapp-put", async ({
  page,
  context,
}) => {
  await page.goto(dappPutUrl);
  const webpage = new Webpage(page, context);

  await test.step("Web-app should open sidepanel via SDK call", async () => {
    await webpage.clickButton("Request Web Proof");

    const extension = await waitForExtension(context);
    expect(extension).toBeDefined();
  });

  const extension = await waitForExtension(context);
  const appPagePromise = extension.redirect();

  await test.step("Extension should stay ok after clicking request button multiple times", async () => {
    await page.goto(dappPutUrl);
    const webpage = new Webpage(page, context);

    await webpage.clickButton("Request Web Proof");
    await webpage.clickButton("Request Web Proof");
    await webpage.clickButton("Request Web Proof");

    const extension = await waitForExtension(context);
    expect(extension).toBeDefined();
  });

  const appPage = await appPagePromise;

  await test.step("On 'redirect' click extension should open new browser tab for specified startPage url", async () => {
    await appPage.waitForURL(loginUrl);
  });

  await test.step("Side panel UI should indicate that startPage step is completed", async () => {
    await extension.expectStepToBeCompleted("startPage");
  });

  await test.step("Side panel UI should indicate that expectUrl step is completed after history.pushState redirect", async () => {
    await appPage.clickButton("Login");
    await appPage.waitForURL(dashboardUrl);

    await extension.expectStepToBeCompleted("expectUrl");
  });

  await test.step("Side panel UI should activate prove button after clicking update resource button", async () => {
    await Promise.all([
      context.waitForEvent(
        "response",
        (response) =>
          response.url().includes("update_resource") &&
          response.status() === 200,
      ),
      appPage.clickButton("Update resource"),
    ]);

    await extension.expectGenerateProofButtonToBeVisible();
  });

  await test.step("Click button should generate webproof", async () => {
    await extension.generateWebProof();

    await webpage.expectWebProof();
  });
});
