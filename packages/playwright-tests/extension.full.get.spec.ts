import { expect, test } from "./config";
import { type Response } from "@playwright/test";
import { loginUrl, dashboardUrl, profileUrl, dappUrl } from "./urls";
import { Webpage } from "./pom/webpage";
import { waitForExtension } from "./pom/extension";

test("Full flow from opening sidepanel to redirection", async ({
  page,
  context,
}) => {
  await test.step("Web-app should open sidepanel via SDK call", async () => {
    await page.goto(dappUrl);
    const webpage = new Webpage(page, context);

    await webpage.clickButton("Request proof of being a wizard");

    const extension = await waitForExtension(context);
    expect(extension).toBeDefined();
  });

  await page.goto(dappUrl);
  const webpage = new Webpage(page, context);

  const extension = await waitForExtension(context);
  const appPagePromise = extension.redirect();

  await test.step("Extension should stay ok after clicking request button multiple times", async () => {
    await webpage.clickButton("Request proof of being a wizard");
    await webpage.clickButton("Request proof of being a wizard");
    await webpage.clickButton("Request proof of being a wizard");

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

  await test.step("Side panel UI should indicate that expectUrl step is completed after redirection", async () => {
    await appPage.clickButton(new RegExp("^Go to profile$"));
    await appPage.waitForURL(profileUrl);

    await extension.expectStepToBeCompleted("expectUrl", 1);
  });

  await test.step("Prove button should appear after request to external api", async () => {
    await extension.expectGenerateProofButtonToBeVisible();
  });

  await test.step("Click button should generate webproof", async () => {
    await extension.generateWebProof();

    await extension.expectCountDown();

    await webpage.expectWebProof();

    await extension.expectCountDownToBeHidden();
  });

  await test.step("Zk prove button should appear after receiving webProof", async () => {
    await webpage.expectRequestZkProofButtonToBeVisible();
  });

  await test.step("Prove button should disappear after generating webproof", async () => {
    await extension.expectGenerateProofButtonToBeHidden();
  });

  await test.step("Request and response should be displayed with correctly redacted headers", async () => {
    await webpage.expectContainText(
      "redacted-request",
      "accept-encoding: identity",
    );
    await webpage.expectContainText("redacted-request", "connection: *****");

    await webpage.expectContainText(
      "redacted-response",
      "Access-Control-Allow-Methods: GET",
    );
    await webpage.expectContainText(
      "redacted-response",
      "Access-Control-Expose-Headers: **************************************************************************************************************",
    );
    await webpage.expectContainText(
      "redacted-response",
      "Access-Control-Allow-Headers: **************************************************************************************************************",
    );
    await webpage.expectContainText("redacted-response", '"name":"Gandalf"');
    await webpage.expectContainText(
      "redacted-response",
      '"greeting":"*************"',
    );
  });

  await test.step("Proving request has succeeded", async () => {
    const vlayerResponses: Promise<Response | null>[] = [];
    page.on("requestfinished", (req) => vlayerResponses.push(req.response()));

    await webpage.requestZkProof();

    await page.getByText("Has zk proof").waitFor();

    await webpage.expectValidZkProof(vlayerResponses);
  });

  await test.step("Prover returned correctly redacted parts of response body", async () => {
    await webpage.expectTextByTextId("name-from-prover", "Gandalf");
    await webpage.expectTextByTextId("greeting-from-prover", "*************");
  });
});
