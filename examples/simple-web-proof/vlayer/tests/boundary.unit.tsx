/// <reference types="@testing-library/jest-dom" />

import { describe, test, vi, expect, afterEach } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import { ErrorBoundary } from "react-error-boundary";
import "@testing-library/jest-dom/vitest";
import { StepErrorBoundaryComponent } from "../src/components/layout/ErrorBoundary";
import { AlreadyMintedError } from "../src/errors";

const ThrowError = () => {
  throw new Error("Test error");
};

const ThrowAlreadyMintedError = () => {
  throw new AlreadyMintedError();
};

// without this line, test will pass but displays a lot of error messages in the console
vi.spyOn(console, "error").mockImplementation(() => null);

describe("ErrorBoundary", () => {
  afterEach(() => {
    cleanup(); // Clears the DOM after each test
  });

  test("should render error boundary component when there is an error", () => {
    render(
      <ErrorBoundary FallbackComponent={StepErrorBoundaryComponent}>
        <ThrowError />
      </ErrorBoundary>,
    );
    const errorDisplayed = screen.queryByTestId("Error display");
    expect(errorDisplayed).toBeInTheDocument();
  });

  test("should not render error boundary component when there is no error", () => {
    render(
      <ErrorBoundary FallbackComponent={StepErrorBoundaryComponent}>
        <h1>OK</h1>
      </ErrorBoundary>,
    );
    const errorDisplayed = screen.queryByTestId("Error display");
    expect(errorDisplayed).not.toBeInTheDocument();
  });

  test("should display default error message when unknown error", () => {
    render(
      <ErrorBoundary FallbackComponent={StepErrorBoundaryComponent}>
        <ThrowError />
      </ErrorBoundary>,
    );
    const errorDisplayed = screen.queryByText("Something went wrong");
    expect(errorDisplayed).toBeInTheDocument();
  });

  test("should display custom error message when known error", () => {
    render(
      <ErrorBoundary FallbackComponent={StepErrorBoundaryComponent}>
        <ThrowAlreadyMintedError />
      </ErrorBoundary>,
    );
    const errorDisplayed = screen.queryByText(
      "NFT has already been minted for this account.",
    );
    expect(errorDisplayed).toBeInTheDocument();
  });
});
