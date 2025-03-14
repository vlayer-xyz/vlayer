import { describe, test, vi, expect, afterEach } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import { ErrorBoundary } from "react-error-boundary";
import "@testing-library/jest-dom/vitest";
import { ErrorBoundaryComponent } from "../src/components/layout/ErrorBoundary";

const ThrowError = () => {
  throw new Error("Test error");
};

const Ok = () => {
  return <h1>OK</h1>;
};

// without this line, test will pass but display "Error: Uncaught [Error: Test error]"
vi.spyOn(console, "error").mockImplementation(() => null);

describe("ErrorBoundary", () => {
  afterEach(() => {
    cleanup(); // Clears the DOM after each test
  });

  test("should render error boundary component when there is an error", () => {
    render(
      <ErrorBoundary FallbackComponent={ErrorBoundaryComponent}>
        <ThrowError />
      </ErrorBoundary>,
    );
    const errorDisplayed = screen.queryByTestId("Error display");
    expect(errorDisplayed).toBeInTheDocument();
  });

  test("should not render error boundary component when there is no error", () => {
    render(
      <ErrorBoundary FallbackComponent={ErrorBoundaryComponent}>
        <Ok />
      </ErrorBoundary>,
    );
    const errorDisplayed = screen.queryByTestId("Error display");
    expect(errorDisplayed).not.toBeInTheDocument();
  });
});
