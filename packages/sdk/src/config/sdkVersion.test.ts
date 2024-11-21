import { describe, expect, test } from "vitest";
import { SDK_VERSION } from "./sdkVersion";

describe("SDK version", () => {
  test("Uninitialized SDK version", () => {
    expect(SDK_VERSION).toStrictEqual({
      version: "0.0.0",
      image_id: "IMAGE_ID_PLACEHOLDER",
    });
  });
});
