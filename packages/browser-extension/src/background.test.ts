import "./background";
import { describe, it, expect, beforeEach, vi } from "vitest";
import { zkProvingStatusManager } from "./state/zkStatus";
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
    const zkProvingSpy = vi.spyOn(zkProvingStatusManager, "setProvingStatus");
    await browser.runtime.sendMessage({
      action: ExtensionAction.NotifyZkProvingStatus,
      payload: ZkProvingStatus.Proving,
    });
    expect(zkProvingSpy).toHaveBeenCalledWith(ZkProvingStatus.Proving);
  });
});
