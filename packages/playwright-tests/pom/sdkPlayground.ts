import { Page } from "@playwright/test";
import { Webpage } from "./webpage";
import { ExtensionMessage, ExtensionMessageType } from "../web-proof-commons";

declare global {
  interface Window {
    _vlayer: {
      extensionWebProofProvider: {
        openSidePanel: () => void;
        closeSidePanel: () => void;
        addEventListeners: (
          event: ExtensionMessageType,
          listener: (
            args: Extract<ExtensionMessage, { type: ExtensionMessageType }>,
          ) => void,
        ) => void;
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

  get extension() {
    return window._vlayer.extensionWebProofProvider;
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

  waitForSidePanelClosedEvent() {
    return new Promise((resolve) => {
      void this.page.evaluate(() => {
        window._vlayer.extensionWebProofProvider.addEventListeners(
          // @ts-expect-error - we are using a custom event
          "SidePanelClosed",
          () => {
            console.log("SidePanelClosed event received");
            resolve(true);
          },
        );
      });
    });
  }
}
