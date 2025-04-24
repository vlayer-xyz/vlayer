import React from "react";
import {
  render,
  screen,
  act,
  cleanup,
  within,
  renderHook,
} from "@testing-library/react";
import { vi, beforeEach, describe, afterEach, expect, it } from "vitest";
import { RedirectCallout } from "./RedirectCallout";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";
import { ExtensionInternalMessageType } from "../../../../web-proof-commons";
import { DEFAULT_REDIRECT_DELAY_SECONDS } from "constants/defaults";
import {
  CALLOUT_DEBOUNCE_TIME,
  useNotarizeStepActions,
} from "./NotarizeStepActions.hooks";
import { StepStatus } from "constants/step";

const mocks = vi.hoisted(() => ({
  useTlsnProver: vi.fn(),
  useZkProvingState: vi.fn(),
}));

vi.mock("hooks/useZkProvingState", () => ({
  useZkProvingState: mocks.useZkProvingState,
}));

vi.mock("hooks/useTlsnProver", () => ({
  useTlsnProver: mocks.useTlsnProver,
}));

vi.mock("lib/sendMessageToServiceWorker", () => ({
  default: vi.fn().mockImplementation(() => Promise.resolve()),
}));

const defaultProps = {
  buttonText: "click me",
  link: "https://example.com",
  isVisited: false,
  status: StepStatus.Current,
};

const renderRedirectCallout = (show: boolean, timeout: number) => {
  return render(<RedirectCallout show={show} timeout={timeout} />);
};

const setupNotarizeStepActions = () => {
  return renderHook(() => useNotarizeStepActions(defaultProps));
};

describe("RedirectCallout", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.clearAllTimers();
    cleanup();
  });

  it("should render correctly when visible", () => {
    renderRedirectCallout(true, DEFAULT_REDIRECT_DELAY_SECONDS);
    expect(
      screen.getByText(/You will be redirected back in/i),
    ).toBeInTheDocument();
  });

  it("should count down and send message to service worker", () => {
    mocks.useTlsnProver.mockReturnValue({
      isProving: true,
      error: null,
    });

    mocks.useZkProvingState.mockReturnValue({
      isProving: false,
      isDone: false,
      error: null,
    });

    const { result, rerender: rerenderHook } = setupNotarizeStepActions();
    const { rerender: rerenderComponent } = renderRedirectCallout(
      result.current.isRedirectCalloutVisible,
      result.current.redirectTimeout,
    );

    act(() => {
      vi.advanceTimersByTime(CALLOUT_DEBOUNCE_TIME);
    });

    act(() => {
      rerenderHook();
      rerenderComponent(
        <RedirectCallout
          show={result.current.isRedirectCalloutVisible}
          timeout={result.current.redirectTimeout}
        />,
      );
    });

    const redirectMessage = screen.getByText(/You will be redirected back in/i);
    const timoutText = within(redirectMessage).getByTestId("timeout");

    expect(timoutText).toHaveTextContent(
      (DEFAULT_REDIRECT_DELAY_SECONDS - 1).toString(),
    );

    for (let i = DEFAULT_REDIRECT_DELAY_SECONDS; i > 0; i--) {
      act(() => {
        vi.advanceTimersByTime(1000);
        rerenderHook();
        rerenderComponent(
          <RedirectCallout
            show={result.current.isRedirectCalloutVisible}
            timeout={result.current.redirectTimeout}
          />,
        );
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
    renderRedirectCallout(false, DEFAULT_REDIRECT_DELAY_SECONDS);
    expect(
      screen.queryByText(/You will be redirected back in/i),
    ).not.toBeInTheDocument();
  });
});
