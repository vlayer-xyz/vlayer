import { expect, test } from "./config";
import { sidePanel } from "./helpers";

test("web proof flow", async ({ page, context }) => {
  await context.addCookies([
    {
      name: "auth_token",
      value: "4469c0e31f1e2054cd3ac1a6e468e2fba0bfc41e",
      path: "/",
      domain: ".x.com",
    },
  ]);

  await test.step("Click Start", async () => {
    await page.goto("/");

    await expect(page.getByTestId("start-page-button")).toBeVisible();
    await page.getByTestId("start-page-button").click();

    await expect(page).toHaveURL("/connect-wallet");
  });

  await test.step("Click Start Proving", async () => {
    await expect(page.getByText("Start Proving")).toBeVisible();
    await page.getByText("Start Proving").click();

    await expect(page).toHaveURL("/start-proving");
  });

  await test.step("Open extension", async () => {
    await expect(page.getByText("Open Extension")).toBeVisible();
    await page.getByText("Open Extension").click();

    const extension = await sidePanel(context);
    expect(extension).toBeDefined();
  });

  await test.step("Redirect to x.com", async () => {
    const extension = await sidePanel(context);
    expect(extension).toBeDefined();

    const redirectButton = extension.getByTestId("start-page-button");
    const [newPage] = await Promise.all([
      context.waitForEvent("page"),
      redirectButton.click(),
    ]);

    await expect(newPage).toHaveURL("https://x.com/home");
  });
});
