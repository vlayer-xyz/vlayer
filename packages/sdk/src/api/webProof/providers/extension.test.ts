import {
  ExtensionAction,
  ExtensionMessageType,
  ZkProvingStatus,
} from "src/web-proof-commons";
import { createExtensionWebProofProvider, EXTENSION_ID } from "./extension";
import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => {
  return {
    sendMessage: vi.fn(),
    postMessage: vi
      .fn()
      .mockImplementation(
        (message: {
          action: ExtensionAction;
          payload: { version: string };
        }) => {
          if (message.action === ExtensionAction.RequestVersion) {
            setTimeout(() => {
              callbacks.forEach((callback) => {
                callback({
                  type: ExtensionMessageType.Version,
                  payload: { version: "1.0.0" },
                });
              });
            }, 100);
          }
        },
      ),
  };
});

const callbacks: ((message: {
  type: ExtensionMessageType;
  payload: { version: string };
}) => void)[] = [];

vi.useFakeTimers();

vi.stubGlobal("chrome", {
  runtime: {
    connect: vi.fn().mockImplementation(() => {
      return {
        onMessage: {
          addListener: vi
            .fn()
            .mockImplementation(
              (
                callback: (message: {
                  type: ExtensionMessageType;
                  payload: { version: string };
                }) => void,
              ) => {
                console.log("onMessage", callback);
                callbacks.push(callback);
              },
            ),
        },
        postMessage: mocks.postMessage,
      };
    }),
    sendMessage: mocks.sendMessage,
  },
});

describe.only("ExtensionWebProofProvider with extension not installed", () => {
  beforeEach(() => {
    mocks.postMessage.mockImplementation(() => {});
  });

  it("should return null version if there is no extension", async () => {
    const provider = createExtensionWebProofProvider();
    vi.advanceTimersByTime(1000);
    await expect(provider.getExtensionVersion()).resolves.toBeNull();
  });

  it("shouldnt notify about zk-proving status", () => {
    const provider = createExtensionWebProofProvider();
    vi.advanceTimersByTime(1000);
    provider.notifyZkProvingStatus(ZkProvingStatus.Proving);
    expect(mocks.sendMessage).not.toHaveBeenCalled();
  });
});

describe("ExtensionWebProofProvider with extension installed", () => {
  beforeEach(() => {
    mocks.postMessage = vi
      .fn()
      .mockImplementation(
        (message: {
          action: ExtensionAction;
          payload: { version: string };
        }) => {
          if (message.action === ExtensionAction.RequestVersion) {
            setTimeout(() => {
              callbacks.forEach((callback) => {
                callback({
                  type: ExtensionMessageType.Version,
                  payload: { version: "1.0.0" },
                });
              });
            }, 100);
          }
        },
      );
  });

  it("should try to connect to extension just after creation", () => {
    createExtensionWebProofProvider();
    expect(chrome.runtime.connect).toHaveBeenCalled();
  });

  it("should properly get extension version", async () => {
    const provider = createExtensionWebProofProvider();
    vi.advanceTimersByTime(1000);
    await expect(provider.getExtensionVersion()).resolves.toBe("1.0.0");
  });

  it("should notify about zk-proving status", () => {
    const provider = createExtensionWebProofProvider();
    vi.advanceTimersByTime(2000);
    provider.notifyZkProvingStatus(ZkProvingStatus.Proving);
    expect(mocks.sendMessage).toHaveBeenCalledWith(EXTENSION_ID, {
      action: ExtensionAction.NotifyZkProvingStatus,
      payload: { status: ZkProvingStatus.Proving },
    });
  });
});
