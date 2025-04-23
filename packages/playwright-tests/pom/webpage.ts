import { BrowserContext, Page } from "@playwright/test";
import { MessageToExtensionType, ZkProvingStatus } from "../web-proof-commons";
import { extensionId, expect } from "../config";
import { type Response } from "@playwright/test";

//Webpage acts as a webpage that uses SDK to communicate with extension
export class Webpage {
  constructor(
    protected readonly page: Page,
    protected readonly context: BrowserContext,
  ) {}
  async waitForURL(url: string) {
    await this.page.waitForURL(url);
  }

  clickButton(name?: string | RegExp) {
    return this.page.getByRole("button", { name }).click();
  }

  private sendMessageToExtension(type: string, payload?: object) {
    return this.page.evaluate(
      ({
        type,
        extensionId,
        payload,
      }: {
        type: string;
        extensionId: string;
        payload?: object;
      }) => {
        void chrome.runtime.sendMessage(extensionId, {
          type,
          ...payload,
        });
      },
      { type, extensionId, payload },
    );
  }

  openExtension() {
    return this.sendMessageToExtension(MessageToExtensionType.OpenSidePanel);
  }

  closeExtension() {
    return this.sendMessageToExtension(MessageToExtensionType.CloseSidePanel);
  }

  finishZkProof() {
    return this.sendMessageToExtension(
      MessageToExtensionType.NotifyZkProvingStatus,
      {
        payload: { status: ZkProvingStatus.Done },
      },
    );
  }

  async expectRequestZkProofButtonToBeVisible() {
    const button = this.page.getByRole("button", { name: "Request zk proof" });
    await expect(button).toBeVisible();
  }

  async requestZkProof() {
    const button = this.page.getByRole("button", { name: "Request zk proof" });
    await button.click();
  }

  async expectWebProof() {
    await this.page.getByText("Has web proof").waitFor();
  }

  async expectZkProof() {
    await this.page.getByText("Has zk proof").waitFor();
  }

  async expectTextByTextId(id: string, expectedText: string) {
    const text = this.page.locator("body").getByTestId(id);
    expect(await text.textContent()).toEqual(expectedText);
  }

  async expectContainText(id: string, expectedText: string) {
    const text = this.page.locator("body").getByTestId(id);
    await expect(text).toBeVisible();

    expect(await text.textContent()).toContain(expectedText);
  }

  async expectValidZkProof(vlayerResponses: Promise<Response | null>[]) {
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
  }
}
