import { Benchmark } from "../types";
import spec from "../../../../contracts/fixtures/out/ArithOpProver.sol/ArithOpProver";

export const benchmarks: Benchmark[] = [
  { name: "Addition", spec, args: [1, 2], functionName: "add" },
  { name: "Multiplication", spec, args: [1, 2], functionName: "mul" },
  { name: "Subtraction", spec, args: [1, 2], functionName: "sub" },
  { name: "Division", spec, args: [4, 2], functionName: "div" },
  { name: "Signed-division", spec, args: [-4, 2], functionName: "sdiv" },
];
