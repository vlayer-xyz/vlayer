import { render, screen, cleanup } from "@testing-library/react";
import { vi, describe, it, expect, beforeEach, afterEach } from "vitest";
import { NotarizeStepActions } from "./NotarizeStepActions";
import { StepStatus } from "constants/step";

import React from "react";

const mocks = vi.hoisted(() => {
  return {
    useTlsnProver: vi.fn(),
    useZkProvingState: vi.fn(),
  };
});

vi.mock("hooks/useZkProvingState", () => ({
  useZkProvingState: mocks.useZkProvingState,
}));

vi.mock("hooks/useTlsnProver", () => ({
  useTlsnProver: mocks.useTlsnProver,
}));

describe("NotarizeStepActions", () => {
  beforeEach(() => {
    cleanup();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("should render webProvingIndicator when needed", () => {
    mocks.useZkProvingState.mockReturnValue({
      isProving: false,
    });
    mocks.useTlsnProver.mockReturnValue({
      isProving: true,
    });
    render(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );
    const webProvingIndicator = screen.getByTestId("step_proving_web");
    expect(webProvingIndicator).toBeInTheDocument();
  });

  it("should render zkProvingIndicator when needed", () => {
    mocks.useZkProvingState.mockReturnValue({
      isProving: true,
    });

    mocks.useTlsnProver.mockReturnValue({
      isProving: false,
    });

    render(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );
    const zkProvingIndicator = screen.getByTestId("step_proving_zk");
    expect(zkProvingIndicator).toBeInTheDocument();
  });

  it("should render Generate proof button when needed", () => {
    mocks.useZkProvingState.mockReturnValue({
      isProving: false,
    });
    mocks.useTlsnProver.mockReturnValue({
      isProving: false,
    });
    render(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );
    const button = screen.getByTestId("prove-button");
    expect(button).toBeInTheDocument();
  });

  it("should render progress 100% when zkProving is done", () => {
    vi.useFakeTimers();
    mocks.useZkProvingState.mockReturnValue({
      isProving: true,
    });
    mocks.useTlsnProver.mockReturnValue({
      isProving: false,
    });
    const { rerender } = render(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );
    mocks.useZkProvingState.mockReturnValue({
      isProving: false,
      isDone: true,
    });
    rerender(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );
    const progressBar = screen.getByTestId("proving-progress");
    expect(progressBar.getAttribute("data-value")).toBe("100");
  });

  it("should hide progress bar when zkProving is done", () => {
    mocks.useZkProvingState.mockReturnValue({
      isProving: true,
    });
    mocks.useTlsnProver.mockReturnValue({
      isProving: false,
    });
    const { rerender } = render(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );
    mocks.useZkProvingState.mockReturnValue({
      isProving: false,
      isDone: true,
    });
    rerender(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );

    const progressBar = screen.getByTestId("proving-progress");
    expect(progressBar).not.toBeVisible();
  });
  it("should render finish callout when proof is generated", () => {
    mocks.useZkProvingState.mockReturnValue({
      isProving: false,
      isDone: true,
    });
    render(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );
    const finishCallout = screen.getByTestId("finish-callout");
    expect(finishCallout).toBeInTheDocument();
    expect(finishCallout).toHaveTextContent(
      "Generating proof has been finished",
    );
  });

  it("should hide finish callout", () => {
    mocks.useZkProvingState.mockReturnValue({
      isProving: false,
      isDone: true,
    });
    render(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );
    const finishCallout = screen.getByTestId("finish-callout");
    expect(finishCallout).toBeInTheDocument();
    expect(finishCallout).toHaveTextContent(
      "Generating proof has been finished",
    );
    // act(() => {
    //   vi.advanceTimersByTime(3000);
    // });
    expect(finishCallout).not.toBeVisible();
  });
});
