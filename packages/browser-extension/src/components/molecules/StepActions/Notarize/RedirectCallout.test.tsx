import React from "react";
import { render, screen, act, cleanup, within } from "@testing-library/react";
import { vi, beforeEach, describe, afterEach, expect, it } from "vitest";
import { RedirectCallout } from "./RedirectCallout";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";
import { ExtensionInternalMessageType } from "../../../../web-proof-commons";
import { DEFAULT_REDIRECT_DELAY_SECONDS } from "constants/defaults";

vi.mock("lib/sendMessageToServiceWorker", () => ({
  default: vi.fn().mockImplementation(() => Promise.resolve()),
}));

describe("RedirectCallout", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.clearAllTimers();
    cleanup();
  });

  it("should render correctly when visible", () => {
    render(<RedirectCallout isVisible={true} />);
    expect(
      screen.getByText(/You will be redirected back in/i),
    ).toBeInTheDocument();
  });

  it("should count down and send message to service worker", () => {
    render(<RedirectCallout isVisible={true} />);
    const redirectMessage = screen.getByText(/You will be redirected back in/i);
    const timoutText = within(redirectMessage).getByTestId("timeout");
    expect(timoutText).toHaveTextContent(
      DEFAULT_REDIRECT_DELAY_SECONDS.toString(),
    );

    for (let i = DEFAULT_REDIRECT_DELAY_SECONDS; i > 0; i--) {
      act(() => {
        vi.advanceTimersByTime(1000);
      });
      expect(timoutText).toHaveTextContent((i - 1).toString());
    }

    act(() => {
      vi.advanceTimersByTime(1000);
    });

    expect(sendMessageToServiceWorker).toHaveBeenCalledWith({
      type: ExtensionInternalMessageType.RedirectBack,
    });
  });

  it("should not render when not visible", () => {
    render(<RedirectCallout isVisible={false} />);
    expect(
      screen.queryByText(/You will be redirected back in/i),
    ).not.toBeInTheDocument();
  });
});
