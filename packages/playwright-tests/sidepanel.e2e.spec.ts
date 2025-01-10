import { expect, test } from "./config";
import { sidePanel } from "./helpers";
import { Response } from "@playwright/test";

const config = {
  loginUrl: "/login",
  profileUrl: "/profile",
  dashboardUrl: "/dashboard",
};

test.describe("Full flow of webproof using extension", () => {
  test("Full flow from opening sidepanel to redirection using proveWeb", async ({
    page,
    context,
  }) => {
    await test.step("Web-app should open sidepanel via SDK call", async () => {
      await page.goto("/dapp-prove-web");
      const requestProofButton = page
        .locator("body")
        .getByTestId("zk-prove-button");

      await requestProofButton.click();
      const extension = await sidePanel(context);
      expect(extension).toBeDefined();
    });

    await test.step("On 'redirect' click extension should open new browser tab for specified startPage url", async () => {
      const extension = await sidePanel(context);

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
      const extension = await sidePanel(context);
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
      const loginButton = startPage.getByTestId("login-button");
      await loginButton.click();
      const extension = await sidePanel(context);
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
      const profileButton = dashboardPage.getByTestId("go-to-profile-button");
      await profileButton.click();
      await dashboardPage.waitForURL(config.profileUrl);
      const extension = await sidePanel(context);
      const startPageStep = extension.getByTestId("step-expectUrl").nth(1);
      const status = await startPageStep.getAttribute("data-status");
      expect(status).toEqual("completed");
    });

    await test.step("Prove button should appear after request to external api", async () => {
      const extension = await sidePanel(context);
      const proveButton = extension.getByTestId("prove-button");
      await expect(proveButton).toHaveText("Generate proof");
    });

    await test.step("Click button should generate both webproof and zkproof", async () => {
      const vlayerResponses: Promise<Response | null>[] = [];
      page.on("requestfinished", (req) => vlayerResponses.push(req.response()));

      const extension = await sidePanel(context);
      const proveButton = extension.getByTestId("prove-button");
      await proveButton.click();
      await page.waitForSelector('h1[data-testid="has-zkproof"]');

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
  });

  test("Full flow from opening sidepanel to redirection", async ({
    page,
    context,
  }) => {
    await test.step("Web-app should open sidepanel via SDK call", async () => {
      await page.goto("/dapp-new-way");
      const requestProofButton = page
        .locator("body")
        .getByTestId("request-webproof-button");

      await requestProofButton.click();
      const extension = await sidePanel(context);
      expect(extension).toBeDefined();
    });

    await test.step("Extension should stay ok after clinking request button multiple times", async () => {
      await page.goto("/dapp-new-way");
      const requestProofButton = page
        .locator("body")
        .getByTestId("request-webproof-button");
      await requestProofButton.click();
      await requestProofButton.click();
      await requestProofButton.click();

      const extension = await sidePanel(context);
      expect(extension).toBeDefined();
      const redirectButton = extension.getByTestId("start-page-button");
      await expect(redirectButton).toBeVisible();
    });

    await test.step("On 'redirect' click extension should open new browser tab for specified startPage url", async () => {
      const extension = await sidePanel(context);

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
      const extension = await sidePanel(context);
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
      const loginButton = startPage.getByTestId("login-button");
      await loginButton.click();
      const extension = await sidePanel(context);
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
      const profileButton = dashboardPage.getByTestId("go-to-profile-button");
      await profileButton.click();
      await dashboardPage.waitForURL(config.profileUrl);
      const extension = await sidePanel(context);
      const startPageStep = extension.getByTestId("step-expectUrl").nth(1);
      const status = await startPageStep.getAttribute("data-status");
      expect(status).toEqual("completed");
    });

    await test.step("Prove button should appear after request to external api", async () => {
      const extension = await sidePanel(context);
      const proveButton = extension.getByTestId("prove-button");
      await expect(proveButton).toHaveText("Generate proof");
    });

    await test.step("Click button should generate webproof", async () => {
      const extension = await sidePanel(context);
      const proveButton = extension.getByTestId("prove-button");
      await proveButton.click();
      await page.reload();
      await page.waitForSelector('h1[data-testid="has-webproof"]');
    });

    await test.step("Zk prove button should appear after receiving webProof", async () => {
      const proveButton = page.locator("body").getByTestId("zk-prove-button");
      await expect(proveButton).toBeVisible();
    });

    await test.step("Proving request has succeeded", async () => {
      const proveButton = page.locator("body").getByTestId("zk-prove-button");

      const vlayerResponses: Promise<Response | null>[] = [];
      page.on("requestfinished", (req) => vlayerResponses.push(req.response()));

      await proveButton.click();

      await page.waitForSelector('h1[data-testid="has-zkproof"]');

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
  });

  test("Full flow from opening sidepanel to redirection with legacy communication", async ({
    page,
    context,
  }) => {
    await test.step("Web-app should open sidepanel via SDK call", async () => {
      await page.goto("/dapp");
      const requestProofButton = page
        .locator("body")
        .getByTestId("request-webproof-button");

      await requestProofButton.click();
      const extension = await sidePanel(context);
      expect(extension).toBeDefined();
    });

    await test.step("On 'redirect' click extension should open new browser tab for specified startPage url", async () => {
      const extension = await sidePanel(context);

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
      const extension = await sidePanel(context);
      const startPageStep = extension.getByTestId("step-startPage");
      const status = await startPageStep.getAttribute("data-status");
      expect(status).toEqual("completed");
    });

    await test.step("Side panel UI shoud indicate that expectUrl step is completed after history.pushState redirect", async () => {
      const startPage = context.pages().find((page) => {
        return page.url().includes(config.loginUrl);
      });
      if (!startPage) {
        throw new Error("No login page");
      }
      const loginButton = startPage.getByTestId("login-button");
      await loginButton.click();
      const extension = await sidePanel(context);
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
      const profileButton = dashboardPage.getByTestId("go-to-profile-button");
      await profileButton.click();
      await dashboardPage.waitForURL(config.profileUrl);
      const extension = await sidePanel(context);
      const startPageStep = extension.getByTestId("step-expectUrl").nth(1);
      const status = await startPageStep.getAttribute("data-status");
      expect(status).toEqual("completed");
    });
    await test.step("Prove button should appear after request to external api", async () => {
      const extension = await sidePanel(context);
      const proveButton = extension.getByTestId("prove-button");
      expect(proveButton).toBeDefined();
    });

    await test.step("Click button should generate webproof", async () => {
      const extension = await sidePanel(context);
      const proveButton = extension.getByTestId("prove-button");
      await proveButton.click();
      await page.waitForSelector('h1[data-testid="has-webproof"]');
    });

    await test.step("Zk prove button should appear after receiving webProof", () => {
      const proveButton = page.locator("body").getByTestId("zk-prove-button");
      void expect(proveButton).toBeVisible();
    });

    await test.step("Proving request has succeeded", async () => {
      const proveButton = page.locator("body").getByTestId("zk-prove-button");

      const vlayerResponses: Promise<Response | null>[] = [];
      page.on("requestfinished", (req) => vlayerResponses.push(req.response()));

      await proveButton.click();

      await page.waitForSelector('h1[data-testid="has-zkproof"]');

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
  });
});
