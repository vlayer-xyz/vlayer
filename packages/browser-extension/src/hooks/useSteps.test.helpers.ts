import { BrowsingHistoryItem } from "../state/history.ts";
import { StepStatus } from "constants/step.ts";
import { expect } from "vitest";
import { calculateSteps } from "./useSteps";
import chalk from "chalk";
import { steps } from "./useSteps.test.data.ts";

export const expectedStatuses = async ({
  input,
  output,
}: {
  input: {
    isZkProvingDone: boolean;
    history: BrowsingHistoryItem[];
  };
  output: StepStatus[];
}) => {
  (
    await calculateSteps({
      stepsSetup: steps,
      ...input,
    })
  ).forEach((step, index) => {
    expect(step.status).toEqual(output[index]);
  });
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
