import { expect } from "playwright/test";
import { test } from "./config";
import { sidePanel } from "./helpers";
import { installMockWallet } from "@johanneskares/wallet-mock";
import { privateKeyToAccount } from "viem/accounts";
import { anvil, optimismSepolia } from "viem/chains";
import { http } from "viem";
import { getConfig } from "@vlayer/sdk/config";

const { privateKey, chainName } = getConfig();
const chain = chainName ? anvil : optimismSepolia;

test.beforeEach(async ({ page }) => {
  await installMockWallet({
    page,
    account: privateKeyToAccount(privateKey),
    defaultChain: chain,
    transports: { [chain.id]: http() },
  });
});

test("web proof flow", async ({ page, context }) => {
  // To obtain this token, log in to x.com and copy the value of the `auth_token` cookie.
  const authToken = process.env.PLAYWRIGHT_TEST_X_COM_AUTH_TOKEN;

  if (!authToken) {
    throw new Error(
      "Missing required environment variable: PLAYWRIGHT_TEST_X_COM_AUTH_TOKEN",
    );
  }

  await context.addCookies([
    {
      name: "auth_token",
      value: authToken,
      path: "/",
      domain: ".x.com",
    },
  ]);

  await test.step("Clicking Start should redirect to Connect Wallet", async () => {
    await page.goto("/");

    await expect(page.getByTestId("start-page-button")).toBeVisible();
    await page.getByTestId("start-page-button").click();

    await expect(page).toHaveURL("/connect-wallet");
  });

  await test.step("Clicking Start Proving should redirect to Start Proving", async () => {
    await expect(page.getByText("Start Proving")).toBeVisible();
    await page.getByText("Start Proving").click();

    await expect(page).toHaveURL("/start-proving");
  });

  await test.step("Clicking Open Extension should make side panel visible", async () => {
    await expect(page.getByText("Open Extension")).toBeVisible();
    await page.getByText("Open Extension").click();

    const extension = await sidePanel(context);
    expect(extension).toBeDefined();
  });

  await test.step("Extension should automatically open x.com", async () => {
    const newPage = await context.waitForEvent("page");

    await expect(newPage).toHaveURL("https://x.com/home");
  });

  await test.step("Clicking Generate proof should succeed and redirect to minting", async () => {
    const extension = await sidePanel(context);
    const proveButton = extension.getByRole("button", {
      name: "Generate proof",
    });
    await proveButton.click();
    await expect(page.getByText("Start Minting")).toBeVisible({
      timeout: 120_000,
    });
  });

  await test.step("Clicking Start Minting should display Success", async () => {
    await page.getByText("Start Minting").click();
    await expect(page.getByText("Success")).toBeVisible();
    expect(await page.getByText("was minted to").textContent()).toMatch(
      /@[\w]+ was minted to 0x[a-fA-F0-9]{4}\.\.\.[a-fA-F0-9]{4}/,
    );
  });
});
