import {
  render,
  screen,
  cleanup,
  renderHook,
  act,
} from "@testing-library/react";
import { vi, describe, it, expect, beforeEach, afterEach } from "vitest";
import { NotarizeStepActions } from "./NotarizeStepActions";
import { StepStatus } from "constants/step";

import React from "react";
import { useNotarizeStepActions } from "./NotarizeStepActions.hooks";
import { CALLOUT_DEBOUNCE_TIME } from "./NotarizeStepActions.hooks";
const mocks = vi.hoisted(() => {
  return {
    useTlsnProver: vi.fn(),
    useZkProvingState: vi.fn(),
  };
});

const SECOND = 1000;

vi.mock("hooks/useZkProvingState", () => ({
  useZkProvingState: mocks.useZkProvingState,
}));

vi.mock("hooks/useTlsnProver", () => ({
  useTlsnProver: mocks.useTlsnProver,
}));

describe("NotarizeStepActions", () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
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
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });
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
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });
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
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });
    const button = screen.getByRole("button", { name: "Generate proof" });
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
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });
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
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });
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
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });
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
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });
    const finishCallout = screen.getByTestId("finish-callout");
    expect(finishCallout).not.toBeVisible();

    expect(finishCallout).toBeInTheDocument();
    expect(finishCallout).toHaveTextContent(
      "Generating proof has been finished",
    );

    expect(finishCallout).not.toBeVisible();
  });
  it("should not render redirect callout when web proving is not started", () => {
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
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });
    const redirectCallout = screen.queryByText(
      /You will be redirected back in/i,
    );
    expect(redirectCallout).not.toBeInTheDocument();
  });

  it("should render redirect callout when web proving is started", () => {
    mocks.useTlsnProver.mockReturnValue({
      isProving: true,
    });

    const { result } = renderHook(() =>
      useNotarizeStepActions({
        isVisited: false,
        status: StepStatus.Current,
        buttonText: "click me ",
        link: "https://example.com",
      }),
    );
    render(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });

    const redirectCallout = screen.getByText(/You will be redirected back in/i);

    expect(result.current.isRedirectCalloutVisible).toBe(true);
    expect(redirectCallout).toBeInTheDocument();
  });
  it("redirect callout should be hidden when error occurs during web proving", () => {
    mocks.useTlsnProver.mockReturnValue({
      isProving: true,
    });

    const { result, rerender } = renderHook(() =>
      useNotarizeStepActions({
        isVisited: false,
        status: StepStatus.Current,
        buttonText: "click me ",
        link: "https://example.com",
      }),
    );
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });
    expect(result.current.isRedirectCalloutVisible).toBe(true);

    render(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });
    mocks.useTlsnProver.mockReturnValue({
      isProving: false,
      error: "Some error occurred during web proving",
    });
    rerender();
    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });
    expect(result.current.isRedirectCalloutVisible).toBe(false);
    const redirectCallout = screen.queryByTestId("redirect-callout");
    expect(redirectCallout).not.toBeVisible();
  });
  it("once rerender, redirect callout should stay visible after web proving finished if no error occurs", () => {
    mocks.useTlsnProver.mockReturnValue({
      isProving: true,
    });

    const { result, rerender } = renderHook(() =>
      useNotarizeStepActions({
        isVisited: false,
        status: StepStatus.Current,
        buttonText: "click me ",
        link: "https://example.com",
      }),
    );
    act(() => {
      vi.advanceTimersByTime(SECOND);
    });
    expect(result.current.isRedirectCalloutVisible).toBe(true);

    render(
      <NotarizeStepActions
        buttonText={"click me "}
        link={"https://example.com"}
        isVisited={false}
        status={StepStatus.Current}
      />,
    );
    act(() => {
      vi.advanceTimersByTime(SECOND);
    });
    rerender();

    expect(result.current.isRedirectCalloutVisible).toBe(true);

    const redirectCallout = screen.getByTestId("redirect-callout");
    expect(redirectCallout).toBeInTheDocument();
  });
});
