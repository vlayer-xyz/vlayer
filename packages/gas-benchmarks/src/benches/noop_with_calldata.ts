import { Benchmark } from "../types";
import proverSpec from "../../../../contracts/fixtures/out/NoopWithCalldataProver.sol/NoopWithCalldataProver";

function encodeArgs(length: number): string {
  let arr = new Uint8Array(length);
  arr.fill(0xaa);
  return "0x" + Buffer.from(arr).toString("hex");
}

function genBenches(): Array<Benchmark> {
  let arr = [];
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
