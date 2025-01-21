import { ContractArg, ContractSpec } from "@vlayer/sdk";

export type Benchmark = {
  name: string;
  spec: ContractSpec;
  args: ContractArg[];
  functionName: string;
};
