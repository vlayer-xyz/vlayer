import { expect, test } from "./config";
import { type Response } from "@playwright/test";
import { loginUrl, profileUrl, dashboardUrl, dappProveWebUrl } from "./urls";
import { Webpage } from "./pom/webpage";
import { waitForExtension } from "./pom/extension";
import { waitForSidePanelOpened } from "./helpers";

test("Full flow from opening sidepanel to redirection using proveWeb", async ({
  page,
  context,
}) => {
  await page.goto(dappProveWebUrl);
  const webpage = new Webpage(page, context);

  await test.step("Web-app should open sidepanel via SDK call", async () => {
    await webpage.clickButton("Request prove web");

    const extension = await waitForSidePanelOpened(context);
    expect(extension).toBeDefined();
  });

  const extension = await waitForExtension(context);
  const appPage = await extension.redirect();

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

  await test.step("Click button should generate both webproof and zkproof", async () => {
    const vlayerResponses: Promise<Response | null>[] = [];
    page.on("requestfinished", (req) => vlayerResponses.push(req.response()));

    await extension.generateWebProof();

    await webpage.expectZkProof();

    await webpage.expectValidZkProof(vlayerResponses);
  });

  await test.step("Prover returned correctly redacted parts of response body", async () => {
    await webpage.expectTextByTextId("name-from-prover", "Gandalf");
    await webpage.expectTextByTextId("greeting-from-prover", "*************");
  });
});
