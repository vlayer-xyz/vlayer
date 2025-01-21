import { Benchmark } from "../types";
import spec from "../../../../contracts/fixtures/out/NoopProver.sol/NoopProver";

export const benchmark: Benchmark = {
  name: "No-op",
  spec,
  args: [],
  functionName: "noop",
};
