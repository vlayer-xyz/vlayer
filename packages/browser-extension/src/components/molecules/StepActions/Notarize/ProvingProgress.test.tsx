import React from "react";
import { render, screen, cleanup, act } from "@testing-library/react";
import { describe, it, expect, vi, afterEach, beforeEach } from "vitest";
import { ProvingProgress } from "./ProvingProgress";
import { ProvingStatus } from "./types";

describe("ProvingProgress", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });
  afterEach(() => {
    vi.useRealTimers();
    cleanup();
  });
  it("renders the component with initial state", () => {
    render(
      <ProvingProgress isVisible={true} provingStatus={ProvingStatus.Web} />,
    );
    expect(screen.getByText("Generating Web Proof")).toBeInTheDocument();
    expect(
      screen.getByText("This takes a while. Don’t close your browser."),
    ).toBeInTheDocument();
    expect(screen.getByText("Step 1 of 2")).toBeInTheDocument();
    expect(screen.getByTestId("proving-progress")).toHaveAttribute(
      "data-value",
      "0",
    );
  });

  it("updates progress to 100 when proving status is Done", () => {
    render(
      <ProvingProgress isVisible={true} provingStatus={ProvingStatus.Done} />,
    );
    act(() => {
      vi.advanceTimersByTime(2000);
    });
    expect(screen.getByRole("progressbar")).toHaveAttribute(
      "data-value",
      "100",
    );
  });

  it("updates title and subtitle based on proving status", () => {
    const { rerender } = render(
      <ProvingProgress isVisible={true} provingStatus={ProvingStatus.Web} />,
    );
    expect(screen.getByText("Generating Web Proof")).toBeInTheDocument();
    expect(
      screen.getByText("This takes a while. Don’t close your browser."),
    ).toBeInTheDocument();

    rerender(
      <ProvingProgress isVisible={true} provingStatus={ProvingStatus.Zk} />,
    );
    expect(screen.getByText("Generating ZK Proof")).toBeInTheDocument();
    expect(
      screen.getByText("This takes a while. Don’t close your browser."),
    ).toBeInTheDocument();
  });

  it("increments progress over time", () => {
    render(
      <ProvingProgress isVisible={true} provingStatus={ProvingStatus.Web} />,
    );

    act(() => {
      vi.advanceTimersByTime(2400);
    });

    expect(screen.getByRole("progressbar")).toHaveAttribute("data-value", "1");

    act(() => {
      vi.advanceTimersByTime(2400);
    });

    expect(screen.getByRole("progressbar")).toHaveAttribute("data-value", "2");
  });
});
