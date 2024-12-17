import { createExtensionWebProofProvider } from "./extension";
import { describe, it, expect, vi } from "vitest";
import { expectUrl, startPage, notarize } from "../steps";
import { StepValidationError } from "../../../web-proof-commons";

const chrome = {
  runtime: {
    disconnectCallbacks: [] as (() => void)[],
    connect: vi.fn().mockImplementation(() => ({
      postMessage: vi.fn().mockImplementation(() => {}),
      onMessage: {
        addListener: vi.fn().mockImplementation(() => {}),
      },
      onDisconnect: {
        addListener: vi.fn().mockImplementation((callback: () => void) => {
          chrome.runtime.disconnectCallbacks.push(callback);
        }),
      },
    })),
    disconnect: vi.fn().mockImplementation(() => {
      chrome.runtime.disconnectCallbacks.forEach((callback) => {
        callback();
      });
    }),
  },
};

vi.stubGlobal("chrome", chrome);

const defaults = {
  logoUrl: "https://example.com/logo.png",
  proverCallCommitment: {
    address: "0x" as `0x${string}`,
    proverAbi: [],
    chainId: 1,
    functionName: "test",
    commitmentArgs: null as never,
  },
};

const invalidUrl = "http:/example.com";
const invalidUrlPattern = "http://+.test";
const validUrl = "https://example.com";
const validUrlPattern = "https://example.com/test";
const label = "test";

describe("ExtensionWebProofProvider", () => {
  it("should properly validate startPage step", () => {
    const provider = createExtensionWebProofProvider();
    expect(() =>
      provider.requestWebProof({
        ...defaults,
        steps: [startPage(invalidUrl, label)],
      }),
    ).toThrow(StepValidationError);
  });

  it("should properly validate expectUrl step", () => {
    const provider = createExtensionWebProofProvider();
    expect(() =>
      provider.requestWebProof({
        ...defaults,
        steps: [expectUrl(invalidUrlPattern, label)],
      }),
    ).toThrow(StepValidationError);
  });

  it("should properly validate notarize step", () => {
    const provider = createExtensionWebProofProvider();
    expect(() =>
      provider.requestWebProof({
        ...defaults,
        steps: [notarize(invalidUrlPattern, "GET", label)],
      }),
    ).toThrow(StepValidationError);
  });

  it("successfully validates all steps", () => {
    const provider = createExtensionWebProofProvider();
    expect(() =>
      provider.requestWebProof({
        ...defaults,
        steps: [
          startPage(validUrl, label),
          expectUrl(validUrlPattern, label),
          notarize(validUrlPattern, "GET", label),
        ],
      }),
    ).not.toThrow(StepValidationError);
  });

  it("should properly work backward compatible way with only urls used", () => {
    const provider = createExtensionWebProofProvider();

    expect(() =>
      provider.requestWebProof({
        ...defaults,
        steps: [
          startPage(validUrl, label),
          expectUrl(validUrl, label),
          expectUrl(validUrl, label),
          notarize(validUrl, "GET", label),
        ],
      }),
    ).not.toThrow();
  });

  it("should reconnect extension on disconnect", () => {
    const provider = createExtensionWebProofProvider();
    provider.requestWebProof({
      ...defaults,
      steps: [
        startPage(validUrl, label),
        expectUrl(validUrlPattern, label),
        notarize(validUrlPattern, "GET", label),
      ],
    });
    chrome.runtime.connect.mockClear();
    chrome.runtime.disconnect();
    expect(chrome.runtime.connect).toHaveBeenCalled();
  });
});
