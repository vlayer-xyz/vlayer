import { describe, it, expect, beforeAll } from "vitest";
import { renderHook, act } from "@testing-library/react-hooks";
import { useSteps } from "./useSteps";
import browser from "webextension-polyfill";
import { HistoryItem } from "../state/history.ts";
import {
  WebProofStepStartPage,
  WebProofStepExpectUrl,
  WebProofStepNotarize,
  EXTENSION_STEP,
} from "../web-proof-commons";

import { match } from "ts-pattern";
import { mock, instance, when } from "ts-mockito";
import { StepStatus } from "constants/step.ts";

const mockStartPageStep = mock<WebProofStepStartPage>();
const mockExpectUrlPage = mock<WebProofStepExpectUrl>();
const mockStepNotarizePage = mock<WebProofStepNotarize>();

when(mockStartPageStep.url).thenReturn("https://start.com");
when(mockStartPageStep.label).thenReturn("Start Page Label");
when(mockStartPageStep.step).thenReturn("startPage");

when(mockExpectUrlPage.url).thenReturn("https://expected.com");
when(mockExpectUrlPage.label).thenReturn("https://example.com");
when(mockExpectUrlPage.step).thenReturn("expectUrl");

when(mockStepNotarizePage.url).thenReturn("https://notarize.com");
when(mockStepNotarizePage.label).thenReturn("Start Page Label");
when(mockStepNotarizePage.step).thenReturn("notarize");

const getMockHistoryItemInstance = (
  url: string,
  ready: boolean,
): HistoryItem => {
  return {
    ready,
    url,
  };
};

describe("Use steps hook", () => {
  beforeAll(async () => {
    await browser.storage.local.set({ history: [] });
  });

  it("should initialize steps as empty array", () => {
    const { result } = renderHook(() => useSteps());
    expect(result.current).toEqual([]);
  });
  it("should properly initialize steps", async () => {
    const { result } = renderHook(() => useSteps());
    await act(async () => {
      await browser.storage.local.set({
        webProverSessionConfig: {
          steps: [
            mockStartPageStep,
            mockExpectUrlPage,
            mockStepNotarizePage,
          ].map(instance),
        },
      });
    });

    result.current.forEach((step) => {
      expect(step.status).toEqual(
        step.kind === EXTENSION_STEP.startPage
          ? StepStatus.Current
          : StepStatus.Further,
      );
    });
  });

  it("should update steps on visit start page ", async () => {
    const { result } = renderHook(() => useSteps());
    //visit start page
    await act(async () => {
      const visitedStartPage = getMockHistoryItemInstance(
        instance(mockStartPageStep).url,
        true,
      );
      const currentHistory = await browser.storage.local.get("history");
      await browser.storage.local.set({
        history: (currentHistory["history"] as unknown[]).concat(
          visitedStartPage,
        ),
      });
    });
    result.current.forEach((step) => {
      expect(step.status).toEqual(
        match(step.kind)
          .with(EXTENSION_STEP.startPage, () => StepStatus.Completed)
          .with(EXTENSION_STEP.expectUrl, () => StepStatus.Current)
          .with(EXTENSION_STEP.notarize, () => StepStatus.Further)
          .exhaustive(),
      );
    });
  });

  it("should update steps on visit expected url but waiting for completness ", async () => {
    const { result } = renderHook(() => useSteps());
    //visit start page
    await act(async () => {
      const visitedExpectedPageWithoutCookies = getMockHistoryItemInstance(
        instance(mockExpectUrlPage).url,
        false,
      );
      const currentHistory = await browser.storage.local.get("history");
      await browser.storage.local.set({
        history: (currentHistory["history"] as unknown[]).concat(
          visitedExpectedPageWithoutCookies,
        ),
      });
    });
    result.current.forEach((step) => {
      expect(step.status).toEqual(
        match(step.kind)
          .with(EXTENSION_STEP.startPage, () => StepStatus.Completed)
          .with(EXTENSION_STEP.expectUrl, () => StepStatus.Current)
          .with(EXTENSION_STEP.notarize, () => StepStatus.Further)
          .exhaustive(),
      );
    });
  });

  it("should keep notarize step further till cookies are not there ", async () => {
    const { result } = renderHook(() => useSteps());
    //visit start page
    await act(async () => {
      const visitedExpectedPageWithCookies = getMockHistoryItemInstance(
        instance(mockExpectUrlPage).url,
        true,
      );
      const currentHistory = await browser.storage.local.get("history");
      await browser.storage.local.set({
        history: (currentHistory["history"] as unknown[]).concat(
          visitedExpectedPageWithCookies,
        ),
      });
    });
    result.current.forEach((step) => {
      expect(step.status).toEqual(
        match(step.kind)
          .with(EXTENSION_STEP.startPage, () => StepStatus.Completed)
          .with(EXTENSION_STEP.expectUrl, () => StepStatus.Completed)
          .with(EXTENSION_STEP.notarize, () => StepStatus.Further)
          .exhaustive(),
      );
    });
  });

  it("should make notarize step 'current' when cookies are ready", async () => {
    const { result } = renderHook(() => useSteps());
    //visit start page
    await act(async () => {
      const recorderNotarizeUrl = getMockHistoryItemInstance(
        instance(mockStepNotarizePage).url,
        true,
      );
      const currentHistory = await browser.storage.local.get("history");
      await browser.storage.local.set({
        history: (currentHistory["history"] as unknown[]).concat(
          recorderNotarizeUrl,
        ),
      });
    });
    result.current.forEach((step) => {
      expect(step.status).toEqual(
        match(step.kind)
          .with(EXTENSION_STEP.startPage, () => StepStatus.Completed)
          .with(EXTENSION_STEP.expectUrl, () => StepStatus.Completed)
          .with(EXTENSION_STEP.notarize, () => StepStatus.Current)
          .exhaustive(),
      );
    });
  });
});
