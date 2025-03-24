import { Page } from "@playwright/test";
import { Webpage } from "./webpage";

declare global {
  interface Window {
    _vlayer: {
      extensionWebProofProvider: {
        openSidePanel: () => void;
        closeSidePanel: () => void;
      };
    };
  }
}
export class SdkPlayground extends Webpage {
  constructor(page: Page) {
    super(page);
  }

  init() {
    return this.page.goto("/sdk-playground");
  }

  openSidePanel() {
    return this.page.evaluate(() => {
      return window._vlayer.extensionWebProofProvider.openSidePanel();
    });
  }

  closeSidePanel() {
    return this.page.evaluate(() => {
      return window._vlayer.extensionWebProofProvider.closeSidePanel();
    });
  }
}
