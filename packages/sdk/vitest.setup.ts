import { vi } from "vitest";

vi.doMock("viem", () => {
  return {
    encodeFunctionData: vi.fn().mockImplementation(() => "0x"),
    decodeFunctionResult: vi.fn().mockImplementation(() => "0x"),
  };
});
