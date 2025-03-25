import { test } from "./config";
import { SdkPlayground } from "./pom/sdkPlayground";
import { sidePanel, waitForSidePanelClosed } from "./helpers";

const animationTimeout = 300;
test("sidepanel open/close by sdk", async ({ page, context }) => {
  const sdkPlayground = new SdkPlayground(page);
  await sdkPlayground.init();
  await sdkPlayground.openSidePanel();
  await page.waitForTimeout(animationTimeout);
  await sidePanel(context);
  await sdkPlayground.closeSidePanel();
  await sdkPlayground.waitForSidePanelClosedEvent();
  await waitForSidePanelClosed(context);
});
