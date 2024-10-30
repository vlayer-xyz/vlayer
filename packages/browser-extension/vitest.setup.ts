import { vi } from "vitest";

vi.mock("webextension-polyfill", () => {
  return {
    default: {},
  };
});

vi.mock(import("@vlayer/extension-hooks"), () => {
  return {};
});
