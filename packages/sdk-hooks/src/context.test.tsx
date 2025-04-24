import { describe, it, expect } from "vitest";
import { renderHook } from "@testing-library/react";
import {
  ProofProvider,
  useProofContext,
  useProverContext,
  useWebProofContext,
} from "./context";
import type { PropsWithChildren } from "react";
import { DEFAULT_CONFIG, DEFAULT_CONFIG_ENV } from "./defaults";
import { ProofEnv } from "./types";

describe("Context Providers and Hooks", () => {
  describe("useProofContext", () => {
    it("should throw error when used outside ProofProvider", () => {
      expect(() => {
        renderHook(() => useProofContext());
      }).toThrow("useProofContext must be used within a ProofProvider");
    });

    it("should provide both WebProof and Prover contexts", () => {
      const config = {
        proverUrl: "fake-prover-url",
        notaryUrl: "test-notary-url",
        wsProxyUrl: "test-ws-url",
      };

      const wrapper = ({ children }: PropsWithChildren) => (
        <ProofProvider config={config}>{children}</ProofProvider>
      );

      const { result } = renderHook(() => useProofContext(), { wrapper });
      expect(result.current.webProofProvider).toBeDefined();
      expect(result.current.vlayerClient).toBeDefined();
      expect(result.current.config).toEqual(config);
    });

    it("should use default config when env is provided", () => {
      const config = {
        env: ProofEnv.TESTNET,
        wsProxyUrl: "test-ws-url",
      };

      const wrapper = ({ children }: PropsWithChildren) => (
        <ProofProvider config={config}>{children}</ProofProvider>
      );

      const { result } = renderHook(() => useProofContext(), { wrapper });
      expect(result.current.config).toEqual({
        ...DEFAULT_CONFIG[ProofEnv.TESTNET],
        ...{ wsProxyUrl: config.wsProxyUrl },
      });
    });

    it("should use proper default config when env is not provided", () => {
      const config = {
        proverUrl: "custom-url",
      };

      const wrapper = ({ children }: PropsWithChildren) => (
        <ProofProvider config={config}>{children}</ProofProvider>
      );

      const { result } = renderHook(() => useProofContext(), { wrapper });
      expect(result.current.config).toEqual({
        ...DEFAULT_CONFIG[DEFAULT_CONFIG_ENV],
        ...config,
      });
    });
  });

  describe("useWebProofContext", () => {
    it("should throw error when used outside WebProofProvider", () => {
      expect(() => {
        renderHook(() => useWebProofContext());
      }).toThrow("useWebProofContext must be used within a WebProofProvider");
    });

    it("should return webProofContext when used within Provider", () => {
      const wrapper = ({ children }: PropsWithChildren) => (
        <ProofProvider>{children}</ProofProvider>
      );

      const { result } = renderHook(() => useWebProofContext(), { wrapper });
      expect(result.current.webProofProvider).toBeDefined();
    });
  });

  describe("useProverContext", () => {
    it("should throw error when used outside ProverProvider", () => {
      expect(() => {
        renderHook(() => useProverContext());
      }).toThrow("useProverContext must be used within a ProverProvider");
    });

    it("should return proverContext when used within Provider", () => {
      const wrapper = ({ children }: PropsWithChildren) => (
        <ProofProvider>{children}</ProofProvider>
      );

      const { result } = renderHook(() => useProverContext(), { wrapper });
      expect(result.current.vlayerClient).toBeDefined();
    });
  });
});
