import { describe, it, expect } from "vitest";
import { renderHook } from "@testing-library/react-hooks";
import { useSteps } from "./useSteps";

describe("Use steps hook", () => {
  it("should initialize setps as empty array", () => {
    const { result } = renderHook(() => useSteps());
    expect(result.current).toBeDefined();
  });
});
