import { afterEach, describe, expect, it, vi } from "vitest";
import browser, { type Tabs } from "webextension-polyfill";
import { WebProofStep } from "../../web-proof-commons";
import {
  type InteractiveStepsConfig,
  intoInteractiveStep,
} from "./interactiveSteps";
import { HTTPMethod } from "lib/HttpMethods.ts";
import type { BrowsingHistoryItem } from "src/state";

const stepSetup = ({
  history = [],
  isZkProvingDone = false,
  assertions = {},
  storeAssertion = () => {},
}: Partial<InteractiveStepsConfig>) => ({
  history,
  isZkProvingDone,
  assertions,
  storeAssertion,
});

const makeHistory = (
  items: (string | Partial<BrowsingHistoryItem>)[],
): BrowsingHistoryItem[] =>
  items.map((urlOrItem) => ({
    url: typeof urlOrItem === "string" ? urlOrItem : urlOrItem.url || "",
    ready: typeof urlOrItem === "object" ? urlOrItem.ready : true,
    method: HTTPMethod.GET,
  }));

describe("intoInteractiveStep", () => {
  (["startPage", "redirect", "expectUrl"] as const).map((stepType) => {
    describe(`with ${stepType}`, () => {
      const step = {
        step: stepType,
        url: "https://example.com",
        label: "Start Page",
      } as WebProofStep;

      describe("isReady", () => {
        it("is not set", () => {
          const interactiveStep = intoInteractiveStep(step, stepSetup({}));

          expect(interactiveStep["isReady"]).toBeUndefined();
        });
      });

      describe("isCompleted", () => {
        it("returns true if URL was visited", () => {
          const history = makeHistory([
            "https://another-example.com",
            "https://example.com",
            "https://yet-another-example.com",
          ]);
          const interactiveStep = intoInteractiveStep(
            step,
            stepSetup({ history }),
          );

          expect(interactiveStep.isCompleted?.()).toBe(true);
        });

        it("returns false if URL was not visited", () => {
          const history = makeHistory([
            "https://another-example.com",
            "https://yet-another-example.com",
          ]);
          const interactiveStep = intoInteractiveStep(
            step,
            stepSetup({ history }),
          );

          expect(interactiveStep.isCompleted?.()).toBe(false);
        });
      });
    });
  });

  describe("with notarize", () => {
    const step = {
      step: "notarize",
      url: "https://example.com",
      label: "Notarize Step",
    } as WebProofStep;

    describe("isReady", () => {
      it("returns true if URL request is completed", () => {
        const history = makeHistory([
          { url: "https://another-example.com" },
          { url: "https://example.com", ready: true },
        ]);
        const interactiveStep = intoInteractiveStep(
          step,
          stepSetup({ history }),
        );

        expect(interactiveStep.isReady?.()).toBe(true);
      });

      it("returns false if URL request is not completed", () => {
        const history = makeHistory(["https://another-example.com"]);
        const interactiveStep = intoInteractiveStep(
          step,
          stepSetup({ history }),
        );

        expect(interactiveStep.isReady?.()).toBe(false);
      });

      it("returns false if URL request was performed but is not ready yet", () => {
        const history = makeHistory([
          { url: "https://another-example.com" },
          { url: "https://example.com", ready: false },
        ]);
        const interactiveStep = intoInteractiveStep(
          step,
          stepSetup({ history }),
        );

        expect(interactiveStep.isReady?.()).toBe(false);
      });
    });

    describe("isCompleted", () => {
      it("returns true if zk proving is done", () => {
        const interactiveStep = intoInteractiveStep(
          step,
          stepSetup({ isZkProvingDone: true }),
        );

        expect(interactiveStep.isCompleted?.()).toBe(true);
      });

      it("returns false if zk proving is not done", () => {
        const interactiveStep = intoInteractiveStep(
          step,
          stepSetup({ isZkProvingDone: false }),
        );

        expect(interactiveStep.isCompleted?.()).toBe(false);
      });
    });
  });

  describe("with userAction", () => {
    const step = {
      step: "userAction",
      label: "User Action Step",
      url: "https://example.com",
      instruction: {
        text: "Click the button",
        image: "https://example.com/instruction.png",
      },
      assertion: {
        domElement: "button",
        require: { exist: true, notExist: false },
      },
    } as WebProofStep;

    describe("isReady", () => {
      it("is not set", () => {
        const interactiveStep = intoInteractiveStep(step, stepSetup({}));

        expect(interactiveStep["isReady"]).toBeUndefined();
      });
    });

    describe("isCompleted", () => {
      it("returns true if current URL doesn't match, but has saved positive assertion", async () => {
        const assertions = {
          "User Action Step": true,
        };
        const interactiveStep = intoInteractiveStep(
          step,
          stepSetup({ assertions }),
        );

        expect(await interactiveStep.isCompleted?.()).toBe(true);
      });

      it("returns true if current URL doesn't match, but has saved negative assertion", async () => {
        const assertions = {
          "User Action Step": false,
        };
        const interactiveStep = intoInteractiveStep(
          step,
          stepSetup({ assertions }),
        );

        expect(await interactiveStep.isCompleted?.()).toBe(false);
      });

      it("returns false if current URL doesn't match, and has no matching saved assertions", async () => {
        const interactiveStep = intoInteractiveStep(step, stepSetup({}));

        expect(await interactiveStep.isCompleted?.()).toBe(false);
      });

      it("returns true if current URL matches and element exists", async () => {
        const history = makeHistory(["https://example.com"]);
        const interactiveStep = intoInteractiveStep(
          step,
          stepSetup({ history }),
        );

        // eslint-disable-next-line @typescript-eslint/unbound-method
        vi.mocked(browser.tabs.query).mockResolvedValue([
          { id: "test-id", url: "https://example.com" } as unknown as Tabs.Tab,
        ]);

        document.body.innerHTML = '<button id="test-button">Click me</button>';
        expect(await interactiveStep.isCompleted?.()).toBe(true);
      });

      it("returns false if current URL matches but element does not exist", async () => {
        const history = makeHistory(["https://example.com"]);
        const interactiveStep = intoInteractiveStep(
          step,
          stepSetup({ history }),
        );

        // eslint-disable-next-line @typescript-eslint/unbound-method
        vi.mocked(browser.tabs.query).mockResolvedValue([
          { id: "test-id", url: "https://example.com" } as unknown as Tabs.Tab,
        ]);

        document.body.innerHTML = '<div id="test-div">No button here</div>';
        expect(await interactiveStep.isCompleted?.()).toBe(false);
      });

      afterEach(() => {
        vi.clearAllMocks();
      });
    });
  });
});
