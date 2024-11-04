import { vi } from "vitest";


export const mockStore: () => browser.Storage.StorageArea = function () {
  const store = new Map<string, unknown>();
  const callbacks = new Set<(change: { [key: string]: unknown }) => unknown>();

  return {
    __store__ : store, 
    get: vi.fn().mockImplementation(async function (key: string) {
      const value = store.get(key); 
      return  Promise.resolve({ [key]: value })
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
