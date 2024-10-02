import { WebProof } from "types/webProof.ts";
import { AbiFunction, Hex, Abi, ContractFunctionName } from "viem";
import { Branded } from "types/utils.ts";
import type { ContractFunctionArgsWithout } from "./viem";

export const EXTENSION_STEP = {
  expectUrl: "expectUrl",
  startPage: "startPage",
  notarize: "notarize",
} as const;

export const EXTENSION_ACTION = {
  openSidePanel: "open_side_panel",
} as const;

export const EXTENSION_MESSAGE = {
  proofDone: "proof_done",
  proofError: "proof_error",
} as const;

type ExtensionStep = (typeof EXTENSION_STEP)[keyof typeof EXTENSION_STEP];

export type WebProofStepNotarize = Branded<
  {
    url: string;
    method: string;
    label: string;
    step: typeof EXTENSION_STEP.notarize;
  },
  "notarize"
>;

export type WebProofStepExpectUrl = Branded<
  {
    url: string;
    label: string;
    step: typeof EXTENSION_STEP.expectUrl;
  },
  "expectUrl"
>;

export type WebProofStepStartPage = Branded<
  {
    url: string;
    label: string;
    step: typeof EXTENSION_STEP.startPage;
  },
  "startPage"
>;

export type WebProofSetupInput = {
  logoUrl: string;
  steps: [WebProofStepExpectUrl, WebProofStepStartPage, WebProofStepStartPage];
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
