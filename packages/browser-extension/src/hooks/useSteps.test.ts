import { describe, it } from "vitest";
import { testData } from "./useSteps.test.data.ts";
import { testTitle, expectedStatuses } from "hooks/useSteps.test.helpers.ts";
import { compose, forEach } from "ramda";
import { HistoryItem } from "../state/history.ts";
import { StepStatus } from "constants/step.ts";

const createTest = compose(
  ({
    input,
    output,
  }: {
    input: { proof: object | null; history: HistoryItem[]; id: string };
    output: StepStatus[];
  }) =>
    it(testTitle({ input, output }), () => expectedStatuses({ input, output })),
);

describe("calculateSteps unit", () => {
  forEach(createTest, testData);
});
