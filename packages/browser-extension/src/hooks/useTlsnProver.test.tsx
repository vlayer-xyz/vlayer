import { renderHook, act } from "@testing-library/react";
import { TlsnProofContextProvider, useTlsnProver } from "./useTlsnProver";
import React from "react";
import { describe, it, expect } from "vitest";

describe("useTlsnProver", () => {
  describe("resetTlsnProving", () => {
    it("should reset all states to their initial values", () => {
      const wrapper = ({ children }: { children: React.ReactNode }) => (
        <TlsnProofContextProvider>{children}</TlsnProofContextProvider>
      );
      const { result, rerender: rerenderHook } = renderHook(
        () => useTlsnProver(),
        {
          wrapper,
        },
      );
      act(() => {
        result.current.proof = { someProof: "data" };
        result.current.isProving = true;
        result.current.error = "Some error occurred";
      });

      expect(result.current.proof).toStrictEqual({ someProof: "data" });
      expect(result.current.isProving).toBeTruthy();
      expect(result.current.error).toBe("Some error occurred");

      act(() => {
        result.current.resetTlsnProving();
      });

      act(() => {
        rerenderHook();
      });

      expect(result.current.proof).toBeNull();
      expect(result.current.isProving).toBeFalsy();
      expect(result.current.error).toBeNull();
    });
  });
});
