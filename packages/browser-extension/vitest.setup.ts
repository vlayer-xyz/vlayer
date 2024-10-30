import { vi } from "vitest";

const mockStore = function () {
  const store = new Map<string, unknown>();
  const callbacks = new Set<(change: { [key: string]: unknown }) => unknown>();
  return {
    get: vi.fn().mockImplementation(async function (key: string) {
      return Promise.resolve({ key: store.get(key) });
    }),
    set: vi.fn().mockImplementation(async (key: string, value: unknown) => {
      if (store.has(key)) {
        callbacks.forEach((callback) => {
          callback({ [key]: { newValue: value } });
        });
      }
      store.set(key, value);
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
  return {
    default: {
      storage: {
        local: mockStore(),
        sync: mockStore(),
        session: mockStore(),
      },
    },
  };
});
