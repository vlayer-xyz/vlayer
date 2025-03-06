import { render, screen } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { ErrorCalloutPresentational } from "./ErrorCallout";
import React from "react";
import { SidePanelContainer } from "components/pages/SidePanelContent";

const mocks = vi.hoisted(() => ({
  useTlsnProver: vi.fn(),
}));

vi.mock("hooks/useTlsnProver", () => ({
  useTlsnProver: mocks.useTlsnProver,
}));

describe("ErrorCalloutPresentational", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("does not render when not visible", () => {
    mocks.useTlsnProver.mockReturnValue({ error: "Test error message" });
    render(
      <ErrorCalloutPresentational
        isVisible={false}
        errorMessage="Test error message"
      />,
    );

    expect(screen.queryByText("Test error message")).not.toBeInTheDocument();
  });

  it("renders error message when visible", () => {
    mocks.useTlsnProver.mockReturnValue({ error: "Test error message" });
    render(
      <ErrorCalloutPresentational
        isVisible={true}
        errorMessage="Test error message"
      />,
    );

    expect(screen.getByText("Test error message")).toBeInTheDocument();
  });

  it("renders when tlsn fails to start", () => {
    mocks.useTlsnProver.mockReturnValue({ error: "Test error message" });
    render(<SidePanelContainer />);
    expect(screen.getByText("Test error message")).toBeInTheDocument();
  });
});
