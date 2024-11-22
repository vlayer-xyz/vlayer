import { vi } from "vitest";
import { MessageToExtension } from "./src/web-proof-commons";
import "@testing-library/jest-dom/vitest";

const mockStore = function () {
  const store = new Map<string, unknown>();
  const callbacks = new Set<(change: { [key: string]: unknown }) => unknown>();
  return {
    get: vi.fn().mockImplementation(async function (key: string) {
      return Promise.resolve({ [key]: store.get(key) });
    }),
    set: vi.fn().mockImplementation(async (keys: Record<string, unknown>) => {
      Object.keys(keys).forEach((key: string) => {
        const value = keys[key];
        if (store.has(key)) {
          callbacks.forEach((callback) => {
            callback({ [key]: { newValue: value } });
          });
        }
        store.set(key, value);
      });
      return Promise.resolve();
    }),
    remove: vi.fn().mockImplementation(async (key: string) => {
      store.delete(key);
      return Promise.resolve();
    }),
    clear: vi.fn().mockImplementation(async () => {
      store.clear();
      return Promise.resolve();
    }),
    onChanged: {
      addListener: vi
        .fn()
        .mockImplementation(
          (callback: (changes: { [key: string]: unknown }) => void) => {
            callbacks.add(callback);
          },
        ),
      removeListener: vi
        .fn()
        .mockImplementation(
          (callback: (changes: { [key: string]: unknown }) => void) => {
            callbacks.delete(callback);
          },
        ),
    },
  };
};

const messageCallbacks: ((message: MessageToExtension) => void)[] = [];
const connectionCallbacks: ((message: object) => void)[] = [];
const portCallbacks: ((message: MessageToExtension) => void)[] = [];

export const mockPort = {
  postMessage: vi.fn().mockImplementation(() => {
    console.log("reply to extension");
  }),
  onMessage: {
    addListener: vi
      .fn()
      .mockImplementation((callback: (message: MessageToExtension) => void) => {
        portCallbacks.push(callback);
      }),
  },
};

export const context = {
  connectExternal: vi.fn().mockImplementation(() => {
    connectionCallbacks.forEach((callback) => {
      callback(mockPort);
    });
    return mockPort;
  }),
  sendMessageFromWebpage: vi
    .fn()
    .mockImplementation((message: MessageToExtension) => {
      portCallbacks.forEach((callback) => {
        callback(message);
      });
    }),
};

vi.stubGlobal("scrollTo", vi.fn());

vi.doMock("webextension-polyfill", () => {
  return {
    default: {
      storage: {
        local: mockStore(),
        sync: mockStore(),
        session: mockStore(),
      },
      tabs: {
        query: vi.fn().mockImplementation(() => {
          return Promise.resolve([{ windowId: 0 }]);
        }),
        onActivated: {
          addListener: vi.fn().mockImplementation(() => {}),
        },
      },
      runtime: {
        onInstalled: {
          addListener: vi.fn().mockImplementation(() => {}),
        },
        onConnectExternal: {
          addListener: vi
            .fn()
            .mockImplementation((callback: (message: object) => void) => {
              connectionCallbacks.push(callback);
            }),
          postMessage: vi.fn(),
        },
        onMessage: {
          addListener: vi.fn().mockImplementation(() => {}),
        },

        sendMessage: vi
          .fn()
          .mockImplementation((message: MessageToExtension) => {
            messageCallbacks.forEach((callback) => {
              callback(message);
            });
          }),
        onMessageExternal: {
          addListener: vi
            .fn()
            .mockImplementation(
              (callback: (message: MessageToExtension) => void) => {
                messageCallbacks.push(callback);
              },
            ),
        },
      },
    },
  };
});
