import "./background";
import { describe, it, expect, vi } from "vitest";
import { zkProvingStatusStore } from "./state/zkProvingStatusStore.ts";
import browser from "webextension-polyfill";
import { MessageToExtensionType, ZkProvingStatus } from "./web-proof-commons";

describe("zk related messaging", () => {
  it("should listen to zk proving status messages ", async () => {
    const zkProvingSpy = vi.spyOn(zkProvingStatusStore, "setProvingStatus");
    await window.externalMessageProducer.sendMessage({
      type: MessageToExtensionType.NotifyZkProvingStatus,
      payload: { status: ZkProvingStatus.Proving },
    });
    expect(zkProvingSpy).toHaveBeenCalledWith({
      status: ZkProvingStatus.Proving,
    });
    const storedStatus = await browser.storage.session.get("zkProvingStatus");
    expect(storedStatus.zkProvingStatus).toBe(ZkProvingStatus.Proving);
  });

  it("should clear history and zkProvingStatus on RequestWebProof message", async () => {
    await browser.storage.session.set({ browsingHistory: [{ id: "1" }] });
    await browser.storage.session.set({
      zkProvingStatus: ZkProvingStatus.Proving,
    });
    await window.externalMessageProducer.sendMessage({
      type: MessageToExtensionType.RequestWebProof,
      payload: { steps: [] },
    });
    const storedHistory = await browser.storage.session.get("browsingHistory");
    expect(storedHistory.browsingHistory).toEqual([]);
    const storedStatus = await browser.storage.session.get("zkProvingStatus");
    expect(storedStatus.zkProvingStatus).toEqual(ZkProvingStatus.NotStarted);
  });
});
