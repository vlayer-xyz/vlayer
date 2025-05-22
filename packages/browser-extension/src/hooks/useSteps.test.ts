import { describe, it } from "vitest";
import { testData } from "./useSteps.test.data.ts";
import { expectedStatuses } from "./useSteps.test.helpers.ts";

describe("calculateSteps unit", () => {
  testData.forEach((testCase) => {
    it(testCase.input.id, () => {
      expectedStatuses(testCase);
    });
  });
});
