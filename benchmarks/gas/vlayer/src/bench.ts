import { createVlayerClient, ProofReceipt } from "@vlayer/sdk";
import { getConfig, createContext, deployProver } from "@vlayer/sdk/config";
import { MetricsUnpacked, MetricsStats, Benchmark } from "./types";
import { noop } from "./benches/noop";

export const runBenchmark = async ({
  bench,
  numberOfRepetitions = 10,
}: {
  bench: Benchmark;
  numberOfRepetitions?: number;
}): Promise<MetricsStats> => {
  const config = getConfig();
  const { chain, proverUrl } = createContext(config);

  const prover = await deployProver({
    proverSpec: bench.spec,
    proverArgs: bench.args,
  });

  const vlayer = createVlayerClient({
    url: proverUrl,
  });

  let out_metrics = new MetricsUnpacked();

  const callback = ({ metrics }: ProofReceipt) => {
    out_metrics.gas.push(metrics.gas);
    out_metrics.cycles.push(metrics.cycles);
    out_metrics.times.preflight.push(metrics.times.preflight);
    out_metrics.times.proving.push(metrics.times.proving);
  };

  for (let i = 0; i < numberOfRepetitions; i++) {
    const hash = await vlayer.prove({
      address: prover,
      proverAbi: bench.spec.abi,
      functionName: bench.functionName,
      args: bench.args,
      chainId: chain.id,
    });

    await vlayer.waitForProvingResult({
      hash,
      callback,
    });
  }

  return out_metrics.toStats();
};

let benchmarks = [{ name: "No-op", bench: noop }];
let allStats: MetricsStats[] = [];

for (const { bench } of benchmarks) {
  allStats.push(await runBenchmark({ bench }));
}

console.log("Benchmark results:");

for (let i in benchmarks) {
  const name = benchmarks[i].name;
  const stats = allStats[i];
  console.log(`


      ==============  ${name}  ========================
      Gas:  ${stats.gas.mean} +/- ${stats.gas.stddev}
      Cycles:  ${stats.cycles.mean} +/- ${stats.cycles.stddev}
      ===============================================


  `);
}
