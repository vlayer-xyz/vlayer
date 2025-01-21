import { Metrics } from "@vlayer/sdk";
import { prove, waitForProof } from "../../sdk/src/api/prover";
import { getConfig, createContext, deployProver } from "@vlayer/sdk/config";
import { Benchmark } from "./types";
import { benchmark as noopBenchmark } from "./benches/noop";
import { benchmarks as noopWithCalldataBenchmarks } from "./benches/noop_with_calldata";

const benchmarks = [noopBenchmark, ...noopWithCalldataBenchmarks];

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

const allMetrics: Metrics[] = [];

for (const bench of benchmarks) {
  const metrics: Metrics = await run(bench);
  allMetrics.push(metrics);
}

console.log("Benchmark results:");

for (let i = 0; i < benchmarks.length; i++) {
  const name = benchmarks[i].name;
  const stats = allMetrics[i];
  console.log(`


      ==============  ${name}  ========================
      Gas:  ${stats.gas}
      Cycles:  ${stats.cycles}
      ===============================================


  `);
}
