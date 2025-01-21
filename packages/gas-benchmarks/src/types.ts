import { ContractArg, ContractSpec } from "@vlayer/sdk";

export type Benchmark = {
  name: string;
  spec: ContractSpec;
  args: ContractArg[];
  functionName: string;
};

export type GasWithCycles = {
  gas: number;
  cycles: number;
};
