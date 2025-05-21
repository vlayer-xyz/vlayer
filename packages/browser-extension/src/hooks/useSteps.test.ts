import { describe, expect, it, vi } from "vitest";
import { steps, testData } from "./useSteps.test.data.ts";
import { expectedStatuses } from "hooks/useSteps.test.helpers.ts";
import browser from "webextension-polyfill";
import { useNotifyOnStepCompleted } from "hooks/useSteps.ts";
import { renderHook } from "@testing-library/react";
import { Step, StepStatus } from "src/constants";
import { ExtensionInternalMessageType } from "src/web-proof-commons/types/message.ts";

describe("calculateSteps unit", () => {
  testData.forEach((testCase) => {
    it(testCase.input.id, () => {
      expectedStatuses(testCase);
    });
  });
});

describe("sending message on step completion", () => {
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

    let expectedSentMessages = 0;
    expect(messageSenderSpy).toHaveBeenCalledTimes(0);

    for (let i = 1; i < testData.length; i++) {
      rerender({ currentSteps: intoCurrentStepsProgress(i) });
      const currentlyCompletedSteps = testData[i].output.filter(
        (status) => status === StepStatus.Completed,
      ).length;

      // Should not resend event after we had regression in completed steps
      expectedSentMessages = Math.max(
        currentlyCompletedSteps,
        expectedSentMessages,
      );
      expect(messageSenderSpy).toHaveBeenCalledTimes(expectedSentMessages);
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
