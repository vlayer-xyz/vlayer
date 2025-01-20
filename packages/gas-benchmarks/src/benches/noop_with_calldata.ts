import { Benchmark } from "../types";
import { bytesToHex } from "viem";
import proverSpec from "../../../../contracts/fixtures/out/NoopWithCalldataProver.sol/NoopWithCalldataProver";

function encodeArgs(length: number): string {
  const arr = new Uint8Array(length);
  arr.fill(0xaa);
  return bytesToHex(arr);
}

function genBenches(): Array<Benchmark> {
  const arr = [];
  const calldata_lengths = [1, 2, 3, 4, 10, 20, 100, 1000];
  for (const length of calldata_lengths) {
    arr.push({
      name: `No-op-with-${length}byte-calldata`,
      spec: proverSpec,
      args: [encodeArgs(length)],
      functionName: "noopWithCalldata",
    });
  }
  return arr;
}

export const benchmarks = genBenches();
