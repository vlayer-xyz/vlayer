import { BrowsingHistoryItem } from "../state/history.ts";
import { StepStatus } from "constants/step.ts";
import { expect, vi } from "vitest";
import { calculateSteps } from "./useSteps";
import chalk from "chalk";
import { steps } from "./useSteps.test.data.ts";
import browser, { type Tabs } from "webextension-polyfill";

type TestActiveTab = Partial<Tabs.Tab> & {
  innerHTML?: string;
};

export type StepTestCase = {
  input: {
    id: string;
    isZkProvingDone: boolean;
    history: BrowsingHistoryItem[];
    activeTabContext?: TestActiveTab;
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
    await calculateSteps({
      stepsSetup: steps,
      ...input,
    })
  ).forEach((step, index) => {
    expect(step.status).toEqual(output[index]);
  });

  vi.clearAllMocks();
};

export const testTitle = ({
  input,
  output,
}: {
  output: StepStatus[];
  input: { id: string };
}) => {
  return `should return ${chalk.blue(`[${output.map((e) => e.toString()).join(", ")}]`)} for input ${chalk.blue(input.id)}`;
};
