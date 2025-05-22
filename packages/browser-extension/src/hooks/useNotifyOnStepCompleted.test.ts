import { describe, expect, it, vi } from "vitest";
import browser from "webextension-polyfill";
import { renderHook } from "@testing-library/react";
import { ExtensionInternalMessageType } from "../web-proof-commons";
import { steps, testData } from "./useSteps.test.data.ts";
import { useNotifyOnStepCompleted } from "./useNotifyOnStepCompleted.ts";
import { Step, StepStatus } from "src/constants";

describe("useNotifyOnStepCompleted", () => {
  function intoCurrentStepsProgress(idx: number) {
    return testData[idx].output.map((status) => ({ status })) as Step[];
  }

  it("fires event once for each completed step", () => {
    const messageSenderSpy = vi.spyOn(browser.runtime, "sendMessage");
    const { rerender } = renderHook(
      ({ currentSteps }) => useNotifyOnStepCompleted(steps, currentSteps),
      {
        initialProps: { currentSteps: intoCurrentStepsProgress(0) },
      },
    );

    let expectedSentMessagesCount = 0;
    expect(messageSenderSpy).toHaveBeenCalledTimes(0);

    for (let i = 1; i < testData.length; i++) {
      rerender({ currentSteps: intoCurrentStepsProgress(i) });
      const currentlyCompletedSteps = testData[i].output.filter(
        (status) => status === StepStatus.Completed,
      ).length;

      // Should not resend event after we had regression in completed steps
      expectedSentMessagesCount = Math.max(
        currentlyCompletedSteps,
        expectedSentMessagesCount,
      );
      expect(messageSenderSpy).toHaveBeenCalledTimes(expectedSentMessagesCount);
    }

    expect(messageSenderSpy).toHaveBeenCalledTimes(steps.length);
    for (let i = 0; i < steps.length; i++) {
      expect(messageSenderSpy).nthCalledWith(i + 1, {
        type: ExtensionInternalMessageType.StepCompleted,
        payload: {
          index: i,
          step: steps[i],
        },
      });
    }
  });
});
