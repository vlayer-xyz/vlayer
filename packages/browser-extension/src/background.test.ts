import "./background";
import { describe, it, expect, beforeEach, vi } from "vitest";
import { zkProvingStatusStore } from "./state/zkProvingStatusStore.ts";
import browser from "webextension-polyfill";
import { ExtensionAction, ZkProvingStatus } from "./web-proof-commons";

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
    const storedStatus = await browser.storage.local.get("zkProvingStatus");
    expect(storedStatus.zkProvingStatus).toBe(ZkProvingStatus.Proving);
  });

  it("should clear history on RequestWebProof message", async () => {
    await browser.storage.local.set({ history: [{ id: "1" }] });
    await browser.runtime.sendMessage({
      action: ExtensionAction.RequestWebProof,
    });
    const storedHistory = await browser.storage.local.get("history");
    expect(storedHistory.history).toEqual([]);
  });
});
