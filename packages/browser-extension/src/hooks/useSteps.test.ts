import { describe, it } from "vitest";
import { testData } from "./useSteps.test.data.ts";
import { expectedStatuses } from "./useSteps.test.helpers.ts";

describe("calculateSteps unit", () => {
  for (const testCase of testData) {
    it(testCase.input.id, async () => {
      await expectedStatuses(testCase);
    });
  }
});
