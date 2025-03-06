import { vi } from "vitest";
import { MessageToExtension } from "../src/web-proof-commons";
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

vi.doMock("webextension-polyfill", () => {
  const callbacks: ((message: MessageToExtension) => void)[] = [];

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
      sidePanel: {
        setPanelBehavior: vi.fn().mockImplementation(() => {}),
      },
      runtime: {
        onInstalled: {
          addListener: vi.fn().mockImplementation(() => {}),
        },
        onConnectExternal: {
          addListener: vi.fn().mockImplementation(() => {}),
        },
        onMessage: {
          addListener: vi.fn().mockImplementation(() => {}),
        },

        callbacks: [],
        sendMessage: vi
          .fn()
          .mockImplementation((message: MessageToExtension) => {
            callbacks.forEach((callback) => {
              callback(message);
            });
          }),
        onMessageExternal: {
          addListener: vi
            .fn()
            .mockImplementation(
              (callback: (message: MessageToExtension) => void) => {
                callbacks.push(callback);
              },
            ),
        },
      },
    },
  };
});
