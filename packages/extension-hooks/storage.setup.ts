import { vi } from "vitest";

export const mockStore = () => {
  const store = new Map<string, unknown>();
  const callbacks = new Set<(change: { [key: string]: unknown }) => unknown>();

  return {
    get: vi.fn().mockImplementation(async function (key: string) {
      const value = store.get(key);
      return Promise.resolve({ [key]: value });
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
  const messageListeners = new Set<(message: unknown) => void>();

  return {
    default: {
      storage: {
        local: mockStore(),
        sync: mockStore(),
        session: mockStore(),
      },
      runtime: {
        onMessage: {
          addListener: vi
            .fn()
            .mockImplementation((callback: (message: unknown) => void) => {
              messageListeners.add(callback);
            }),
          removeListener: vi
            .fn()
            .mockImplementation((callback: (message: unknown) => void) => {
              messageListeners.delete(callback);
            }),
        },
        sendMessage: vi.fn().mockImplementation((message: unknown) => {
          messageListeners.forEach((listener) => listener(message));
          return Promise.resolve();
        }),
      },
    },
  };
});
