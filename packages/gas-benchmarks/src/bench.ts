import { Metrics } from "@vlayer/sdk";
import { prove, waitForProof } from "@vlayer/sdk/prover";
import { getConfig, createContext, deployProver } from "@vlayer/sdk/config";
import { Benchmark, GasWithCycles } from "./types";
import { benchmark as noopBenchmark } from "./benches/noop";
import { benchmarks as noopWithCalldataBenchmarks } from "./benches/noop_with_calldata";
import { benchmarks as arithOpBenchmarks } from "./benches/arith_ops";
import debug from "debug";

const log = debug("gas-benchmarks");

const benchmarks = [
  noopBenchmark,
  ...noopWithCalldataBenchmarks,
  ...arithOpBenchmarks,
];

type Results = Record<string, GasWithCycles>;

const results: Results = {};

for (const bench of benchmarks) {
  const metrics = await run(bench);
  results[bench.name] = {
    gas: metrics.gas,
    cycles: metrics.cycles,
  };
}

log(JSON.stringify(results));

prettyPrint(results);

async function run(bench: Benchmark): Promise<Metrics> {
  const config = getConfig();
  const { chain, proverUrl } = createContext(config);

  const prover = await deployProver({
    proverSpec: bench.spec,
  });

  const hash = await prove(
    prover,
    bench.spec.abi,
    bench.functionName,
    bench.args,
    chain.id,
    proverUrl,
  );
  const { metrics } = await waitForProof(hash, proverUrl);

  return metrics;
}

function prettyPrint(results: Results) {
  const MIN_CELL_WIDTH = 10;
  const NUM_COLS = 3;

  const cellWidth = Object.entries(results).reduce(
    (acc, [name, stats]) =>
      Math.max(
        acc,
        name.length,
        `${stats.gas}`.length,
        `${stats.cycles}`.length,
      ),
    MIN_CELL_WIDTH,
  );
  const rowWidth = (cellWidth + 2) * NUM_COLS + NUM_COLS + 1;

  const fmtCell = (str: string) => {
    const padding = cellWidth - str.length;
    return str + " ".repeat(padding);
  };

  const fmtRow = (...values: string[]) =>
    "|" + values.map((value) => " " + fmtCell(value) + " ").join("|") + "|\n";

  const header = fmtRow(" ", "Gas", "Cycles");
  const lineSep = "_".repeat(rowWidth) + "\n";

  let fmt = header + lineSep;
  Object.entries(results).forEach(([name, stats]) => {
    fmt += fmtRow(name, `${stats.gas}`, `${stats.cycles}`);
  });
  fmt += lineSep;
  //eslint-disable-next-line no-console
  console.log(fmt);
}
