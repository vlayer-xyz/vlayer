import { Benchmark } from "../types";
import proverSpec from "../../../../contracts/fixtures/out/ArithOpProver.sol/ArithOpProver";

export const benchmarks: Benchmark[] = [
  { name: "Addition", proverSpec, args: [1, 2], functionName: "add" },
  {
    name: "Multiplication",
    proverSpec,
    args: [1, 2],
    functionName: "mul",
  },
  { name: "Subtraction", proverSpec, args: [1, 2], functionName: "sub" },
  { name: "Division", proverSpec, args: [4, 2], functionName: "div" },
  {
    name: "Signed-division",
    proverSpec,
    args: [-4, 2],
    functionName: "sdiv",
  },
];
