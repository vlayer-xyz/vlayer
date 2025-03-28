import { expect, test } from "./config";
import { waitForSidePanelOpened } from "./helpers";
import { type Response } from "@playwright/test";
import { loginUrl, profileUrl, dashboardUrl, dappProveWebUrl } from "./urls";

test("Full flow from opening sidepanel to redirection using proveWeb", async ({
  page,
  context,
}) => {
  await test.step("Web-app should open sidepanel via SDK call", async () => {
    await page.goto(dappProveWebUrl);
    const requestProofButton = page.locator("body").getByRole("button", {
      name: "Request prove web",
    });

    await requestProofButton.click();
    const extension = await waitForSidePanelOpened(context);
    expect(extension).toBeDefined();
  });

  await test.step("On 'redirect' click extension should open new browser tab for specified startPage url", async () => {
    const extension = await waitForSidePanelOpened(context);

    if (!extension) {
      throw new Error("No sidepanel");
    }
    const redirectButton = extension.getByTestId("start-page-button");
    const [newPage] = await Promise.all([
      context.waitForEvent("page"),
      redirectButton.click(),
    ]);

    await newPage.waitForURL(loginUrl);
  });

  await test.step("Side panel UI should indicate that startPage step is completed", async () => {
    const extension = await waitForSidePanelOpened(context);
    const startPageStep = extension.getByTestId("step-startPage");
    const status = await startPageStep.getAttribute("data-status");
    expect(status).toEqual("completed");
  });

  await test.step("Side panel UI should indicate that expectUrl step is completed after history.pushState redirect", async () => {
    const startPage = context.pages().find((page) => {
      return page.url().includes(loginUrl);
    });
    if (!startPage) {
      throw new Error("No login page");
    }
    const loginButton = startPage.getByRole("button", {
      name: "Login",
    });
    await loginButton.click();
    const extension = await waitForSidePanelOpened(context);
    const startPageStep = extension.getByTestId("step-expectUrl").nth(0);
    const status = await startPageStep.getAttribute("data-status");

    expect(status).toEqual("completed");
  });

  await test.step("Side panel UI should indicate that expectUrl step is completed after redirection", async () => {
    const dashboardPage = context.pages().find((page) => {
      return page.url().includes(dashboardUrl);
    });
    if (!dashboardPage) {
      throw new Error("No dashboard page");
    }
    const profileButton = dashboardPage.getByRole("button", {
      name: /^Go to profile$/,
    });
    await profileButton.click();
    await dashboardPage.waitForURL(profileUrl);
    const extension = await waitForSidePanelOpened(context);
    const startPageStep = extension.getByTestId("step-expectUrl").nth(1);
    const status = await startPageStep.getAttribute("data-status");
    expect(status).toEqual("completed");
  });

  await test.step("Prove button should appear after request to external api", async () => {
    const extension = await waitForSidePanelOpened(context);
    const proveButton = extension.getByRole("button", {
      name: "Generate proof",
    });
    await expect(proveButton).toHaveText("Generate proof");
  });

  await test.step("Click button should generate both webproof and zkproof", async () => {
    const vlayerResponses: Promise<Response | null>[] = [];
    page.on("requestfinished", (req) => vlayerResponses.push(req.response()));

    const extension = await waitForSidePanelOpened(context);
    const proveButton = extension.getByRole("button", {
      name: "Generate proof",
    });
    await proveButton.click();
    await page.getByText("Has zk proof").waitFor();

    expect(vlayerResponses.length).toBeGreaterThan(1);

    const proveResponse = (await vlayerResponses[0])!;
    expect(proveResponse.ok()).toBeTruthy();

    const proveJson = (await proveResponse.json())! as object;
    expect(proveJson).toHaveProperty("result");

    const hash = (proveJson as { result: string }).result;
    expect(hash).toBeValidHash();

    const waitForProvingResultResponse = (await vlayerResponses.pop())!;
    expect(waitForProvingResultResponse.ok()).toBeTruthy();

    const proofJson = (await waitForProvingResultResponse.json()) as object;
    expect(proofJson).toMatchObject({
      result: {
        state: "done",
        status: 1,
        metrics: {},
        data: {
          evm_call_result: {},
          proof: {},
        },
      },
    });
  });

  await test.step("Prover returned correctly redacted parts of response body", async () => {
    const nameFromProver = page.locator("body").getByTestId("name-from-prover");
    const greetingFromProver = page
      .locator("body")
      .getByTestId("greeting-from-prover");

    const name = await nameFromProver.textContent();
    const greeting = await greetingFromProver.textContent();

    expect(name).toEqual("Gandalf");
    expect(greeting).toEqual("*************");
  });
});
