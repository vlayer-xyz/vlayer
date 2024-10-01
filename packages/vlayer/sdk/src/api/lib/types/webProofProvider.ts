import { WebProof } from "types/webProof.ts";
import { AbiFunction, Hex, Abi, ContractFunctionName } from "viem";

import { Branded } from "types/utils.ts";
import type { ContractFunctionArgsWithout } from "./viem";

export type WebProofStepNotarize = Branded<
  {
    url: string;
    method: string;
    label: string;
    kind: StepKind.notarize;
  },
  "notarize"
>;

export enum StepKind {
  expectUrl = "expectUrl",
  startPage = "startPage",
  notarize = "notarize",
}

export type WebProofStepExpectUrl = Branded<
  {
    url: string;
    label: string;
    kind: StepKind.expectUrl;
  },
  "expectUrl"
>;

export type WebProofStepStartPage = Branded<
  {
    url: string;
    label: string;
    kind: StepKind.startPage;
  },
  "startPage"
>;

export type WebProofSetupInput = {
  logoUrl: string;
  steps: [WebProofStepExpectUrl, WebProofStepStartPage, WebProofStepStartPage];
  kind: StepKind;
};

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

type GetWebProofArgs<
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
