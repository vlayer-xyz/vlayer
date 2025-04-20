import { type Hex, type Abi, type ContractFunctionName } from "viem";
import type { ContractFunctionArgsWithout } from "./viem";
import {
  type Branded,
  type MessageFromExtension,
  type WebProofStep,
  type ZkProvingStatus,
  MessageFromExtensionType,
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

export type WebProofConfig<T extends Abi, F extends ContractFunctionName<T>> = {
  proverCallCommitment: ProverCallCommitment<T, F>;
} & WebProofRequestInput;

export type WebProofProvider = {
  requestWebProof: <T extends Abi, F extends ContractFunctionName<T>>(
    args: WebProofConfig<T, F>,
  ) => void;

  notifyZkProvingStatus: (status: ZkProvingStatus) => void;

  addEventListeners: <T extends MessageFromExtensionType>(
    messageType: T,
    listener: (args: Extract<MessageFromExtension, { type: T }>) => void,
  ) => void;
};

export type WebProofProviderSetup = {
  notaryUrl?: string;
  wsProxyUrl?: string;
  token?: string;
};
