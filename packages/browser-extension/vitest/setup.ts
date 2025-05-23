import { vi } from "vitest";
import "@testing-library/jest-dom/vitest";
import { type Scripting } from "webextension-polyfill";

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

declare global {
  interface Window {
    externalMessageProducer: {
      sendMessage: (message: unknown) => Promise<void>;
    };
  }
}

const mockRuntime = () => {
  const callbacks: ((message: unknown) => void)[] = [];
  const callbacksExternal: ((message: unknown) => void)[] = [];
  mockExternalMessageProducer(callbacksExternal);
  mockWebextensionPolyfill(callbacksExternal, callbacks);
  mockChromeNamespace();
};

const mockExternalMessageProducer = (
  callbacksExternal: ((message: unknown) => void)[],
) => {
  vi.stubGlobal("externalMessageProducer", {
    sendMessage: vi.fn().mockImplementation((message: unknown) => {
      callbacksExternal.forEach((callback) => {
        callback(message);
      });
    }),
  });
};

const mockChromeNamespace = () => {
  vi.stubGlobal("chrome", {
    sidePanel: {
      open: vi.fn(),
      setPanelBehavior: vi.fn(),
    },
  });
};
const mockWebextensionPolyfill = (
  callbacksExternal: ((message: unknown) => void)[],
  callbacks: ((message: unknown) => void)[],
) => {
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
        scripting: {
          executeScript: vi
            .fn()
            .mockImplementation(
              async ({
                func,
                args,
              }: {
                func: (...args: unknown[]) => unknown;
                args: unknown[];
              }): Promise<Scripting.InjectionResult[]> => [
                await Promise.resolve({ frameId: 0, result: func(...args) }),
              ],
            ),
        },
        sidePanel: {
          setPanelBehavior: vi.fn().mockImplementation(() => {}),
        },
        runtime: {
          connect: vi.fn().mockImplementation(() => {}),
          onConnect: {
            addListener: vi.fn().mockImplementation(() => {}),
          },
          onInstalled: {
            addListener: vi.fn().mockImplementation(() => {}),
          },
          onConnectExternal: {
            addListener: vi.fn().mockImplementation(() => {}),
          },

          callbacks: [],

          sendMessage: vi.fn().mockImplementation((message: unknown) => {
            callbacks.forEach((callback) => {
              callback(message);
            });
          }),

          onMessage: {
            addListener: vi
              .fn()
              .mockImplementation((callback: (message: unknown) => void) => {
                callbacks.push(callback);
              }),
            removeListener: vi
              .fn()
              .mockImplementation((callback: (message: unknown) => void) => {
                callbacks.splice(callbacks.indexOf(callback), 1);
              }),
          },

          onMessageExternal: {
            addListener: vi
              .fn()
              .mockImplementation((callback: (message: unknown) => void) => {
                callbacksExternal.push(callback);
              }),
            removeListener: vi
              .fn()
              .mockImplementation((callback: (message: unknown) => void) => {
                callbacksExternal.splice(
                  callbacksExternal.indexOf(callback),
                  1,
                );
              }),
          },
        },
      },
    };
  });
};

mockRuntime();
