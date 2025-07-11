import { ContractArg, ContractSpec } from "@vlayer/sdk";

export type Benchmark = {
  name: string;
  proverContractSpec: ContractSpec;
  args: ContractArg[];
  functionName: string;
};

export type GasWithCycles = {
  gas: number;
  cycles: number;
};
