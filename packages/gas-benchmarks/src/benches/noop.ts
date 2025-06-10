import { Benchmark } from "../types";
import proverContractSpec from "../../../../contracts/fixtures/out/NoopProver.sol/NoopProver";

export const benchmark: Benchmark = {
  name: "No-op",
  proverContractSpec,
  args: [],
  functionName: "noop",
};
