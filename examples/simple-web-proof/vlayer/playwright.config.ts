import { defineConfig, devices } from "@playwright/test";

const LOCAL_WEB_SERVER_URL = "http://localhost:5173";
console.log("process.env.WEB_SERVER_URL", process.env.WEB_SERVER_URL);
export default defineConfig({
  // Look for test files in the "tests" directory, relative to this configuration file.
  testDir: "tests",

  // Run all tests in parallel.
  fullyParallel: true,

  // Fail the build on CI if you accidentally left test.only in the source code.
  forbidOnly: !!process.env.CI,

  retries: 0,

  // Opt out of parallel tests on CI.
  workers: process.env.CI ? 1 : undefined,

  // Reporter to use
  reporter: "html",

  use: {
    // Base URL to use in actions like `await page.goto("/")`.
    baseURL: process.env.WEB_SERVER_URL || LOCAL_WEB_SERVER_URL,
    trace: "on",
  },
  // Configure projects for major browsers.
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"], channel: "chromium" },
    },
  ],
  // Run your local dev server before starting the tests.
  webServer: process.env.WEB_SERVER_URL
    ? undefined
    : {
        // we want to be able to modify WEB_SERVER_COMMAND when running in CI
        command:
          process.env.WEB_SERVER_COMMAND ||
          "bun run web:" + process.env.VLAYER_ENV,
        url: LOCAL_WEB_SERVER_URL,
        stdout: "pipe",
        stderr: "pipe",
        timeout: 120_000,
      },
  timeout: 120_000,
});
