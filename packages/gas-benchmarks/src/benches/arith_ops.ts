import { Benchmark } from "../types";
import proverContractSpec from "../../../../contracts/fixtures/out/ArithOpProver.sol/ArithOpProver";

export const benchmarks: Benchmark[] = [
  {
    name: "Addition",
    proverContractSpec,
    args: [1, 2],
    functionName: "add",
  },
  {
    name: "Multiplication",
    proverContractSpec,
    args: [1, 2],
    functionName: "mul",
  },
  {
    name: "Subtraction",
    proverContractSpec,
    args: [1, 2],
    functionName: "sub",
  },
  {
    name: "Division",
    proverContractSpec,
    args: [4, 2],
    functionName: "div",
  },
  {
    name: "Signed-division",
    proverContractSpec,
    args: [-4, 2],
    functionName: "sdiv",
  },
];
