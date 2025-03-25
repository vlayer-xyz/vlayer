import "./background";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { zkProvingStatusStore } from "./state/zkProvingStatusStore.ts";
import browser from "webextension-polyfill";
import { ExtensionAction, ZkProvingStatus } from "./web-proof-commons";
import {
  ExtensionMessageType,
  ExtensionMessage,
} from "./web-proof-commons/types/message";

describe.only("background messaging", () => {
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
    //@ts-expect-error mocking
    // eslint-disable-next-line @typescript-eslint/no-unsafe-call
    await browser.runtime.sendMessageExternal({
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
    //@ts-expect-error mocking
    // eslint-disable-next-line @typescript-eslint/no-unsafe-call
    await browser.runtime.sendMessageExternal({
      action: ExtensionAction.RequestWebProof,
      payload: { steps: [] },
    });
    const storedHistory = await browser.storage.session.get("browsingHistory");
    expect(storedHistory.browsingHistory).toEqual([]);
    const storedStatus = await browser.storage.session.get("zkProvingStatus");
    expect(storedStatus.zkProvingStatus).toEqual(ZkProvingStatus.NotStarted);
  });

  it("should pass message back to port", async () => {
    const port = browser.runtime.connect();
    const messages = (
      [
        {
          type: ExtensionMessageType.ProofDone,
          payload: {
            presentationJson: {},
            decodedTranscript: {
              sent: "sent",
              recv: "recv",
            },
          },
        },
        {
          type: ExtensionMessageType.ProofError,
          payload: { error: "test error" },
        },
        { type: ExtensionMessageType.SidePanelClosed },
      ] as ExtensionMessage[]
    ).map(async (message: ExtensionMessage) => {
      const postMessageSpy = vi.spyOn(port, "postMessage");
      await browser.runtime.sendMessage(message);
      expect(postMessageSpy).toHaveBeenCalledWith(message);
    });
    await Promise.all(messages);
  });
});
