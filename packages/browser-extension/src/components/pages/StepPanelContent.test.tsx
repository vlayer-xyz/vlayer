import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { SidePanelContainer } from "./SidePanelContent";
import * as React from "react";
import { ZkProvingStatus } from "src/web-proof-commons";
import { LOADING } from "@vlayer/extension-hooks";

const mocks = vi.hoisted(() => {
  return {
    useProvingSessionConfig: vi.fn(),
    useTlsnProver: vi.fn(),
    isEmptyWebProverSessionConfig: vi.fn(),
    ZkProvingStatus: vi.fn(),
    useSteps: vi.fn(),
  };
});

vi.mock("hooks/useProvingSessionConfig", () => ({
  useProvingSessionConfig: mocks.useProvingSessionConfig,
}));

vi.mock("hooks/useTlsnProver", () => ({
  useTlsnProver: mocks.useTlsnProver,
}));

vi.mock("hooks/useSteps", () => ({
  useSteps: mocks.useSteps,
}));

vi.mock("../../web-proof-commons", () => ({
  isEmptyWebProverSessionConfig: mocks.isEmptyWebProverSessionConfig,
  ZkProvingStatus: mocks.ZkProvingStatus,
}));

describe("SidePanelContent", () => {
  it("shows loading state when config is loading", () => {
    mocks.useProvingSessionConfig.mockReturnValue([LOADING]);
    mocks.isEmptyWebProverSessionConfig.mockReturnValue(false);
    mocks.ZkProvingStatus.mockReturnValue(ZkProvingStatus.NotStarted);
    mocks.useTlsnProver.mockReturnValue({
      error: null,
      isProving: false,
      proof: null,
      prove: vi.fn(),
      resetTlsnProving: vi.fn(),
    });
    render(<SidePanelContainer />);
    expect(screen.getByText("Loading...")).toBeInTheDocument();
  });

  it("shows empty flow card when config is empty", () => {
    vi.resetAllMocks();
    mocks.useProvingSessionConfig.mockReturnValue([]);
    mocks.isEmptyWebProverSessionConfig.mockReturnValue(true);
    mocks.useTlsnProver.mockReturnValue({
      error: null,
      isProving: false,
      proof: null,
      prove: vi.fn(),
      resetTlsnProving: vi.fn(),
    });
    render(<SidePanelContainer />);
    expect(screen.getByTestId("empty-flow-card")).toBeInTheDocument();
  });

  it("shows steps and help section when config has content", () => {
    vi.resetAllMocks();
    mocks.useProvingSessionConfig.mockReturnValue([{ steps: [] }]);
    mocks.isEmptyWebProverSessionConfig.mockReturnValue(false);
    mocks.useSteps.mockReturnValue([]);
    mocks.useTlsnProver.mockReturnValue({
      error: null,
    });
    render(<SidePanelContainer />);
    expect(screen.getByTestId("steps")).toBeInTheDocument();
    //help section
    expect(screen.getByText("Having Trouble?")).toBeInTheDocument();
  });
});
