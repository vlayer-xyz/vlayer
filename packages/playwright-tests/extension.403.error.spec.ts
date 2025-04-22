import { test } from "./config";
import { Webpage } from "./pom/webpage";
import { waitForExtension } from "./pom/extension";
import {
  loginUrl,
  dashboardUrl,
  profileFailedAuthUrl,
  dappFailedAuthUrl,
} from "./urls";

test("Flow from opening sidepanel until 403 from proven endpoint", async ({
  page,
  context,
}) => {
  await page.goto(dappFailedAuthUrl);
  const webpage = new Webpage(page, context);

  await webpage.clickButton("Request proof of being a wizard");

  const extension = await waitForExtension(context);

  const appPage = await extension.redirect();
  await appPage.waitForURL(loginUrl);

  await appPage.clickButton("Login");
  await appPage.waitForURL(dashboardUrl);

  await appPage.clickButton("Go to profile failed auth");
  await appPage.waitForURL(profileFailedAuthUrl);

  await extension.generateWebProof();
  await extension.expectErrorToBeDisplayed(
    "Non 200 response from proven endpoint.",
  );
});
