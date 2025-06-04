import { BrowsingHistoryItem } from "../../state/history";
import { StepStatus } from "constants/step";
import { expect, vi } from "vitest";
import { calculateSteps } from "./useSteps";
import { steps } from "./useSteps.test.data";
import browser, { type Tabs } from "webextension-polyfill";
import { getInteractiveSteps } from "./interactiveSteps";

type TestActiveTab = Partial<Tabs.Tab> & {
  innerHTML?: string;
};

export type StepTestCase = {
  input: {
    id: string;
    isZkProvingDone: boolean;
    history: BrowsingHistoryItem[];
    activeTabContext?: TestActiveTab;
    assertions?: Record<string, boolean>;
  };
  output: StepStatus[];
};

const mockActiveTab = (activeTabContext: TestActiveTab) => {
  vi.clearAllMocks();

  if (activeTabContext.innerHTML) {
    document.body.innerHTML = activeTabContext.innerHTML;
  }
  // eslint-disable-next-line @typescript-eslint/unbound-method
  vi.mocked(browser.tabs.query).mockResolvedValue([
    { id: "test-id", ...activeTabContext } as Tabs.Tab,
  ]);
};

export const expectedStatuses = async ({ input, output }: StepTestCase) => {
  if (input.activeTabContext) {
    mockActiveTab(input.activeTabContext);
  }
  (
    await calculateSteps(
      getInteractiveSteps(steps, {
        assertions: {},
        storeAssertion: () => {},
        ...input,
      }),
    )
  ).forEach((step, index) => {
    expect(step.status).toEqual(output[index]);
  });

  vi.clearAllMocks();
};
