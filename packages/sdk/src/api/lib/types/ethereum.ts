import { type Abi, type Address, type Hex } from "viem";
import { type Branded } from "../../../web-proof-commons";

export type Bytecode = {
  object: Hex;
};

export type ContractSpec = {
  abi: Abi;
  bytecode: Bytecode;
};

export type ContractArg =
  | number
  | string
  | boolean
  | bigint
  | number[]
  | string[]
  | boolean[]
  | bigint[]
  | Address[]
  | (string | bigint)[]
  | (string | bigint)[][];

export type EthereumAddress = Branded<Hex, "EthereumAddress">;
export type EthereumTxHash = Branded<Hex, "EthereumTxHash">;

export function assertEthereumAddress(
  hash: string,
): asserts hash is EthereumAddress {
  const regex = /^(0x)?[0-9a-fA-F]{40}$/;
  if (!regex.test(hash)) {
    throw new Error(`Invalid ethereum account ${hash}`);
  }
}

export function assertEthereumTxHash(
  hash: string,
): asserts hash is EthereumTxHash {
  const regex = /^(0x)?[0-9a-fA-F]{64}$/;
  if (!regex.test(hash)) {
    throw new Error(`Invalid ethereum tx hash ${hash}`);
  }
}
