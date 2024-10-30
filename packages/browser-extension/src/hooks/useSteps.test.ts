// import { render, screen } from "@testing-library/react";
import { describe, it, expect, re } from "vitest";
import { renderHook } from "@testing-library/react-hooks";
import { useSteps } from "./useSteps";
import { vi } from "vitest";

describe("App", () => {
  it("renders headline", () => {
    const { result } = renderHook(() => useSteps());

    expect(result.current).toBeDefined();
  });
});
