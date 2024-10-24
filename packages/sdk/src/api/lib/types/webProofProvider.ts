import { Hex, Abi, ContractFunctionName } from "viem";
import type { ContractFunctionArgsWithout } from "./viem";
import { Branded, WebProof, WebProofStep } from "../../../web-proof-commons";

export type WebProofSetupInput = {
  logoUrl: string;
  steps: WebProofStep[];
};

export type WebProofSetup = Branded<
  WebProofSetupInput & {
    isWebProof: true;
  },
  "webProof"
>;

export type ProverCallCommitment<
  T extends Abi,
  F extends ContractFunctionName<T>,
> = {
  address: Hex;
  proverAbi: T;
  functionName: F;
  commitmentArgs: ContractFunctionArgsWithout<T, F, { name: "webProof" }>;
  chainId: number;
};

export type GetWebProofArgs<
  T extends Abi,
  F extends ContractFunctionName<T>,
> = {
  proverCallCommitment: ProverCallCommitment<T, F>;
} & WebProofSetupInput;

export type WebProofProvider = {
  getWebProof: <T extends Abi, F extends ContractFunctionName<T>>(
    args: GetWebProofArgs<T, F>,
  ) => Promise<WebProof>;
};

export type WebProofProviderSetup = {
  notaryUrl?: string;
  wsProxyUrl?: string;
};
