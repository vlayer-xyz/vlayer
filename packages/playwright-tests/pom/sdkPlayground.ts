import { BrowserContext, Page } from "@playwright/test";
import { Webpage } from "./webpage";
import { waitForSidePanelClosed, waitForSidePanelOpened } from "../helpers";
import { sdkPlaygroundUrl } from "../urls";

declare global {
  interface Window {
    hasSidePanelClosedNotification: boolean;
    _vlayer: {
      extensionWebProofProvider: {
        openSidePanel: () => void;
        closeSidePanel: () => void;
        addEventListeners: (event: string, callback: () => void) => void;
      };
    };
  }
}

export class SdkPlayground extends Webpage {
  constructor(page: Page, context: BrowserContext) {
    super(page, context);
  }

  async init() {
    await this.page.goto(sdkPlaygroundUrl);
  }

  async openSidePanel() {
    await this.page.evaluate(() => {
      return window._vlayer.extensionWebProofProvider.openSidePanel();
    });
  }

  closeSidePanel() {
    return this.page.evaluate(() => {
      return window._vlayer.extensionWebProofProvider.closeSidePanel();
    });
  }

  async listenToSidePanelClosed() {
    await this.page.evaluate(() => {
      window._vlayer.extensionWebProofProvider.addEventListeners(
        "SidePanelClosed",
        () => {
          window.hasSidePanelClosedNotification = true;
        },
      );
    });
  }

  get isSidePanelClosedNotification() {
    return this.page.evaluate(() => {
      return window.hasSidePanelClosedNotification;
    });
  }

  async waitForSidePanelClosedNotification() {
    while (!(await this.isSidePanelClosedNotification)) {
      await this.page.waitForTimeout(1000);
    }

    await this.page.evaluate(() => {
      window.hasSidePanelClosedNotification = false;
    });

    return true;
  }

  async waitForSidePanelClosed() {
    await waitForSidePanelClosed(this.context);
  }

  async waitForSidePanelOpened() {
    await waitForSidePanelOpened(this.context);
  }
}
