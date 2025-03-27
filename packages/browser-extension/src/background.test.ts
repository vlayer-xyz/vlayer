import "./background";
import { describe, it, expect, beforeEach, vi } from "vitest";
import { zkProvingStatusStore } from "./state/zkProvingStatusStore.ts";
import browser from "webextension-polyfill";
import { ExtensionAction, ZkProvingStatus } from "@vlayer/web-proof-commons";

describe("zk related messaging", () => {
  beforeEach(() => {
    global.chrome = {
      //@ts-expect-error mocking
      sidePanel: {
        open: vi.fn(),
      },
      //@ts-expect-error mocking
      runtime: {
        sendMessage: vi.fn(),

        connect: vi.fn().mockImplementation(() => {
          return {
            onMessage: {
              addListener: vi.fn(),
            },
            postMessage: vi.fn(),
          };
        }),
      },
    };
  });

  it("should listen to zk proving status messages ", async () => {
    const zkProvingSpy = vi.spyOn(zkProvingStatusStore, "setProvingStatus");
    await browser.runtime.sendMessage({
      action: ExtensionAction.NotifyZkProvingStatus,
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
    await browser.runtime.sendMessage({
      action: ExtensionAction.RequestWebProof,
      payload: { steps: [] },
    });
    const storedHistory = await browser.storage.session.get("browsingHistory");
    expect(storedHistory.browsingHistory).toEqual([]);
    const storedStatus = await browser.storage.session.get("zkProvingStatus");
    expect(storedStatus.zkProvingStatus).toEqual(ZkProvingStatus.NotStarted);
  });
});
