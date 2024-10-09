import { WebProof } from "types/webProof.ts";
import { AbiFunction, Hex, Abi, ContractFunctionName } from "viem";
import type { ContractFunctionArgsWithout } from "./viem";
import { Branded, WebProofStepExpectUrl, WebProofStepStartPage } from "@vlayer/web-proof-commons";

export type WebProofSetup = Branded<
  WebProofSetupInput & {
    isWebProof: true;
  },
  "webProof"
>;

export type ProverCallCommitment<
  T extends readonly [AbiFunction, ...Abi[number][]],
  F extends ContractFunctionName<T>,
> = {
  address: Hex;
  proverAbi: T;
  functionName: F;
  commitmentArgs: ContractFunctionArgsWithout<T, F, { name: "webProof" }>;
  chainId: number;
};

export type WebProofSetupInput = {
  logoUrl: string;
  steps: [WebProofStepExpectUrl, WebProofStepStartPage, WebProofStepStartPage];
};

export type GetWebProofArgs<
  T extends readonly [AbiFunction, ...Abi[number][]],
  F extends ContractFunctionName<T>,
> = {
  proverCallCommitment: ProverCallCommitment<T, F>;
} & WebProofSetupInput;

export type WebProofProvider = {
  getWebProof: <
    T extends readonly [AbiFunction, ...Abi[number][]],
    F extends ContractFunctionName<T>,
  >(
    args: GetWebProofArgs<T, F>,
  ) => Promise<WebProof>;
};

export type WebProofProviderSetup = {
  notaryUrl?: string;
  wsProxyUrl?: string;
};
