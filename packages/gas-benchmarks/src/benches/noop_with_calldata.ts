import { Benchmark } from "../types";
import { bytesToHex } from "viem";
import spec from "../../../../contracts/fixtures/out/NoopWithCalldataProver.sol/NoopWithCalldataProver";

function encodeArgs(length: number): string {
  const arr = new Uint8Array(length);
  arr.fill(0xaa);
  return bytesToHex(arr);
}

function genBenches(): Array<Benchmark> {
  const calldata_lengths = [1, 2, 3, 4, 10, 20, 100, 1000];
  return calldata_lengths.map((length) => ({
    name: `No-op-with-${length}byte-calldata`,
    spec,
    args: [encodeArgs(length)],
    functionName: "noopWithCalldata",
  }));
}

export const benchmarks = genBenches();
