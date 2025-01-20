import { ContractArg, ContractSpec } from "@vlayer/sdk";

export interface Benchmark {
  name: string;
  spec: ContractSpec;
  args: ContractArg[];
  functionName: string;
}
