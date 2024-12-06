import { Hex, Abi, ContractFunctionName } from "viem";
import type { ContractFunctionArgsWithout } from "./viem";
import {
  Branded,
  ExtensionMessageType,
  ExtensionMessage,
  type PresentationJSON,
  WebProofStep,
  ZkProvingStatus,
} from "../../../web-proof-commons";

export type WebProofRequestInput = {
  logoUrl: string;
  steps: WebProofStep[];
};

export type WebProofRequest = Branded<
  WebProofRequestInput & {
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
} & WebProofRequestInput;

export type WebProofProvider = {
  getWebProof: <T extends Abi, F extends ContractFunctionName<T>>(
    args: GetWebProofArgs<T, F>,
  ) => Promise<PresentationJSON>;

  requestWebProof: <T extends Abi, F extends ContractFunctionName<T>>(
    args: GetWebProofArgs<T, F>,
  ) => void;

  notifyZkProvingStatus: (status: ZkProvingStatus) => void;

  addEventListeners: <T extends ExtensionMessageType>(
    messageType: T,
    listener: (args: Extract<ExtensionMessage, { type: T }>) => void,
  ) => void;
};

export type WebProofProviderSetup = {
  notaryUrl?: string;
  wsProxyUrl?: string;
};
