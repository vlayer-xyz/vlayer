import { type Branded } from "../../../web-proof-commons/utils";
import {
  type Abi,
  type AbiStateMutability,
  type Address,
  type ContractFunctionArgs,
  type ContractFunctionName,
  type ContractFunctionReturnType,
  type Hex,
} from "viem";
import { type WebProofRequest } from "./webProofProvider";
import { type ContractFunctionArgsWithout } from "./viem";
import { z } from "zod";

type Calldata = string;

export type CallParams = {
  to: Address;
  data: Calldata;
  gas_limit: number;
};

export type CallContext = {
  chain_id: number;
};

export type BrandedHash<T, F> = Branded<{ hash: string }, [T, F]>;

export type Proof = {
  seal: {
    verifierSelector: Hex;
    seal: readonly [Hex, Hex, Hex, Hex, Hex, Hex, Hex, Hex];
    mode: number;
  };
  callGuestId: Hex;
  length: bigint;
  callAssumptions: {
    proverContractAddress: Address;
    functionSelector: Hex;
    settleChainId: bigint;
    settleBlockHash: Hex;
    settleBlockNumber: bigint;
  };
};

export const callHashSchema = z.string().startsWith("0x").length(66);
export type CallHash = z.infer<typeof callHashSchema>;

export enum ProofState {
  Queued = "queued",
  AllocateGas = "allocate_gas",
  Preflight = "preflight",
  Proving = "proving",
  Done = "done",
}

export type ProofData = {
  evm_call_result: Hex;
  proof: Proof;
};

export type Metrics = {
  gas: number;
  cycles: number;
  times: {
    preflight: number;
    proving: number;
  };
};

export type ProofDataWithMetrics = {
  data: ProofData;
  metrics: Metrics;
};

export type ProveArgs<T extends Abi, F extends ContractFunctionName<T>> = {
  address: Hex;
  proverAbi: T;
  functionName: F;
  chainId?: number;
  gasLimit?: number;
  args: ContractFunctionArgs<T, AbiStateMutability, F>;
};

export type VlayerClient = {
  prove: <T extends Abi, F extends ContractFunctionName<T>>(
    args: ProveArgs<T, F>,
  ) => Promise<BrandedHash<T, F>>;

  waitForProvingResult: <
    T extends Abi,
    F extends ContractFunctionName<T>,
  >(args: {
    hash: BrandedHash<T, F>;
    numberOfRetries?: number;
    sleepDuration?: number;
  }) => Promise<ContractFunctionReturnType<T, AbiStateMutability, F>>;

  proveWeb: <T extends Abi, F extends ContractFunctionName<T>>(args: {
    address: Hex;
    proverAbi: T;
    functionName: F;
    chainId: number;
    gasLimit?: number;
    args: [
      WebProofRequest,
      ...ContractFunctionArgsWithout<T, F, { name: "webProof" }>,
    ];
  }) => Promise<BrandedHash<T, F>>;
};

export const proofReceiptSchema = z.discriminatedUnion("status", [
  z.object({
    status: z.literal(0),
    error: z.string(),
    data: z.undefined(),
    metrics: z.custom<Metrics>(),
    state: z.enum([
      ProofState.AllocateGas,
      ProofState.Preflight,
      ProofState.Proving,
    ]),
  }),
  z.object({
    status: z.literal(1),
    error: z.undefined(),
    state: z.enum([
      ProofState.Done,
      ProofState.AllocateGas,
      ProofState.Preflight,
      ProofState.Proving,
      ProofState.Queued,
    ]),
    data: z.custom<ProofData>(),
    metrics: z.custom<Metrics>(),
  }),
]);

export type ProofReceipt = z.infer<typeof proofReceiptSchema>;

export const versionsSchema = z.object({
  call_guest_id: z.string(),
  chain_guest_id: z.string(),
  api_version: z.string(),
  risc0_version: z.string(),
});

export type Versions = z.infer<typeof versionsSchema>;
