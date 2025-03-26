import { BrowserContext, Page } from "@playwright/test";
import { ExtensionAction, ZkProvingStatus } from "../web-proof-commons";
import { extensionId } from "../config";
//Webpage acts as a webpage that uses SDK to communicate with extension
export class Webpage {
  constructor(
    protected readonly page: Page,
    protected readonly context: BrowserContext,
  ) {}
  async waitForURL(url: string) {
    await this.page.waitForURL(url);
  }
  clickButton(name?: string) {
    return this.page.getByRole("button", { name }).click();
  }
  private sendMessageToExtension(action: string, payload?: object) {
    return this.page.evaluate(
      ({
        action,
        extensionId,
        payload,
      }: {
        action: string;
        extensionId: string;
        payload?: object;
      }) => {
        void chrome.runtime.sendMessage(extensionId, {
          action,
          ...payload,
        });
      },
      { action, extensionId, payload },
    );
  }

  openExtension() {
    return this.sendMessageToExtension(ExtensionAction.OpenSidePanel);
  }
  closeExtension() {
    return this.sendMessageToExtension(ExtensionAction.CloseSidePanel);
  }
  finishZkProof() {
    return this.sendMessageToExtension(ExtensionAction.NotifyZkProvingStatus, {
      payload: { status: ZkProvingStatus.Done },
    });
  }
}
