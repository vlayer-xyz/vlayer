import { Benchmark } from "../types";
import proverSpec from "../../../out/NoopProver.sol/NoopProver";

export const benchmark: Benchmark = {
  name: "No-op",
  spec: proverSpec,
  args: [],
  functionName: "noop",
};
