import { vi } from "vitest";

import {
  ExtensionAction,
  ExtensionMessageType,
} from "./src/web-proof-commons";

vi.doMock("viem", () => {
  return {
    encodeFunctionData: vi.fn().mockImplementation(() => "0x"),
    decodeFunctionResult: vi.fn().mockImplementation(() => "0x"),
  };
});


