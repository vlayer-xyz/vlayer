import { Benchmark } from "../types";
import proverSpec from "../../../out/NoopProver.sol/NoopProver";

export const noop: Benchmark = {
  spec: proverSpec,
  args: [],
  functionName: "noop",
};
