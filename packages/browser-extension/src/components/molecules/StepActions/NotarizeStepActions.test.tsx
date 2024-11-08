import { render, screen, cleanup, waitFor } from "@testing-library/react";
import { vi, describe, it, expect, beforeEach } from "vitest";
import { NotarizeStepActions } from "./NotarizeStepActions";
import { ZkProvingStatus } from "../../../web-proof-commons";
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

describe.only("NotarizeStepActions", () => {
  beforeEach(() => {
    cleanup();
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

  it("should render progress 100% when zlProving is done", async () => {
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
      value: ZkProvingStatus.Done,
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
    await waitFor(
      () => expect(progressBar.getAttribute("data-value")).toBe("100"),
      { timeout: 1000 },
    );
  });
});
