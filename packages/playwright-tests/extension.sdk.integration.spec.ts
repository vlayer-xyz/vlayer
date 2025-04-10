import { test } from "./config";
import { SdkPlayground } from "./pom/sdkPlayground";
const animationTimeout = 300;

test("sidepanel open/close by sdk", async ({ page, context }) => {
  const sdkPlayground = new SdkPlayground(page, context);
  await sdkPlayground.init();
  await sdkPlayground.listenToSidePanelClosed();
  await sdkPlayground.openSidePanel();
  await page.waitForTimeout(animationTimeout);
  await sdkPlayground.closeSidePanel();
  await sdkPlayground.waitForSidePanelClosedNotification();
  await sdkPlayground.waitForSidePanelClosed();
});
