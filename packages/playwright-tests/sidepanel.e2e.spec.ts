import { expect, test } from "./config";
import { waitForSidePanelOpened } from "./helpers";
import { type Response } from "@playwright/test";

const config = {
  loginUrl: "/login",
  profileUrl: "/profile",
  profileFailedAuthUrl: "/profile-failed-auth",
  dashboardUrl: "/dashboard",
};

test.describe("Full flow of webproof using extension", () => {
  test("Full flow from opening sidepanel to redirection using proveWeb", async ({
    page,
    context,
  }) => {
    await test.step("Web-app should open sidepanel via SDK call", async () => {
      await page.goto("/dapp-prove-web");
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

      await newPage.waitForURL(config.loginUrl);
    });

    await test.step("Side panel UI should indicate that startPage step is completed", async () => {
      const extension = await waitForSidePanelOpened(context);
      const startPageStep = extension.getByTestId("step-startPage");
      const status = await startPageStep.getAttribute("data-status");
      expect(status).toEqual("completed");
    });

    await test.step("Side panel UI should indicate that expectUrl step is completed after history.pushState redirect", async () => {
      const startPage = context.pages().find((page) => {
        return page.url().includes(config.loginUrl);
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
        return page.url().includes(config.dashboardUrl);
      });
      if (!dashboardPage) {
        throw new Error("No dashboard page");
      }
      const profileButton = dashboardPage.getByRole("button", {
        name: /^Go to profile$/,
      });
      await profileButton.click();
      await dashboardPage.waitForURL(config.profileUrl);
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
      const nameFromProver = page
        .locator("body")
        .getByTestId("name-from-prover");
      const greetingFromProver = page
        .locator("body")
        .getByTestId("greeting-from-prover");

      const name = await nameFromProver.textContent();
      const greeting = await greetingFromProver.textContent();

      expect(name).toEqual("Gandalf");
      expect(greeting).toEqual("*************");
    });
  });

  test("Full flow from opening sidepanel to redirection", async ({
    page,
    context,
  }) => {
    await test.step("Web-app should open sidepanel via SDK call", async () => {
      await page.goto("/dapp");
      const requestProofButton = page
        .locator("body")
        .getByTestId("request-webproof-button");

      await requestProofButton.click();
      const extension = await waitForSidePanelOpened(context);
      expect(extension).toBeDefined();
    });

    await test.step("Extension should stay ok after clicking request button multiple times", async () => {
      await page.goto("/dapp");
      const requestProofButton = page
        .locator("body")
        .getByTestId("request-webproof-button");
      await requestProofButton.click();
      await requestProofButton.click();
      await requestProofButton.click();

      const extension = await waitForSidePanelOpened(context);
      expect(extension).toBeDefined();
      const redirectButton = extension.getByTestId("start-page-button");
      await expect(redirectButton).toBeVisible();
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

      await newPage.waitForURL(config.loginUrl);
    });

    await test.step("Side panel UI should indicate that startPage step is completed", async () => {
      const extension = await waitForSidePanelOpened(context);
      const startPageStep = extension.getByTestId("step-startPage");
      const status = await startPageStep.getAttribute("data-status");
      expect(status).toEqual("completed");
    });

    await test.step("Side panel UI should indicate that expectUrl step is completed after history.pushState redirect", async () => {
      const startPage = context.pages().find((page) => {
        return page.url().includes(config.loginUrl);
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
        return page.url().includes(config.dashboardUrl);
      });
      if (!dashboardPage) {
        throw new Error("No dashboard page");
      }
      const profileButton = dashboardPage.getByRole("button", {
        name: /^Go to profile$/,
      });
      await profileButton.click();
      await dashboardPage.waitForURL(config.profileUrl);
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

    await test.step("Click button should generate webproof", async () => {
      const extension = await waitForSidePanelOpened(context);
      const proveButton = extension.getByRole("button", {
        name: "Generate proof",
      });
      await proveButton.click();
      await page.reload();
      await page.getByText("Has web proof").waitFor();
    });

    await test.step("Zk prove button should appear after receiving webProof", async () => {
      const proveButton = page.locator("body").getByRole("button", {
        name: "Request zk proof",
      });
      await expect(proveButton).toBeVisible();
    });

    await test.step("Request and response should be displayed with correctly redacted headers", async () => {
      const redactedRequest = page
        .locator("body")
        .getByTestId("redacted-request");
      const redactedResponse = page
        .locator("body")
        .getByTestId("redacted-response");
      await expect(redactedRequest).toBeVisible();
      await expect(redactedResponse).toBeVisible();

      const requestText = await redactedRequest.textContent();
      const responseText = await redactedResponse.textContent();

      expect(requestText).toContain("accept-encoding: identity");
      expect(requestText).toContain("connection: *****");

      expect(responseText).toContain("Access-Control-Allow-Methods: GET");
      expect(responseText).toContain(
        "Access-Control-Expose-Headers: **************************************************************************************************************",
      );
      expect(responseText).toContain(
        "Access-Control-Allow-Headers: **************************************************************************************************************",
      );

      expect(responseText).toContain('"name":"Gandalf"');
      expect(responseText).toContain('"greeting":"*************"');
    });

    await test.step("Proving request has succeeded", async () => {
      const proveButton = page.locator("body").getByRole("button", {
        name: "Request zk proof",
      });

      const vlayerResponses: Promise<Response | null>[] = [];
      page.on("requestfinished", (req) => vlayerResponses.push(req.response()));

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
      const nameFromProver = page
        .locator("body")
        .getByTestId("name-from-prover");
      const greetingFromProver = page
        .locator("body")
        .getByTestId("greeting-from-prover");

      const name = await nameFromProver.textContent();
      const greeting = await greetingFromProver.textContent();

      expect(name).toEqual("Gandalf");
      expect(greeting).toEqual("*************");
    });
  });

  test("Flow from opening sidepanel until 403 from proven endpoint", async ({
    page,
    context,
  }) => {
    await test.step("Web-app should open sidepanel via SDK call", async () => {
      await page.goto("/dapp-failed-auth");
      const requestProofButton = page
        .locator("body")
        .getByTestId("request-webproof-button");

      await requestProofButton.click();
      const extension = await waitForSidePanelOpened(context);

      const redirectButton = extension.getByTestId("start-page-button");
      const [newPage] = await Promise.all([
        context.waitForEvent("page"),
        redirectButton.click(),
      ]);

      await newPage.waitForURL(config.loginUrl);

      const startPage = context.pages().find((page) => {
        return page.url().includes(config.loginUrl);
      });
      if (!startPage) {
        throw new Error("No login page");
      }
      const loginButton = startPage.getByRole("button", {
        name: "Login",
      });
      await loginButton.click();

      const dashboardPage = context.pages().find((page) => {
        return page.url().includes(config.dashboardUrl);
      });
      if (!dashboardPage) {
        throw new Error("No dashboard page");
      }
      const profileButton = dashboardPage.getByRole("button", {
        name: "Go to profile failed auth",
      });
      await profileButton.click();
      await dashboardPage.waitForURL(config.profileFailedAuthUrl);

      const proveButton = extension.getByRole("button", {
        name: "Generate proof",
      });
      await expect(proveButton).toHaveText("Generate proof");

      await proveButton.click();

      await extension.waitForSelector('p[data-testid="error-message"]');
      const errorMessage = extension.getByTestId("error-message");
      await expect(errorMessage).toHaveText(
        "Authentication failed. Please restart the process.",
      );
    });
  });
});
