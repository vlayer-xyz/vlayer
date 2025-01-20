import { createVlayerClient, ProofReceipt, Metrics } from "@vlayer/sdk";
import { getConfig, createContext, deployProver } from "@vlayer/sdk/config";
import { Benchmark } from "./types";
import { benchmark as noopBenchmark } from "./benches/noop";
import { benchmarks as noopWithCalldataBenchmarks } from "./benches/noop_with_calldata";

export const runBenchmark = async (bench: Benchmark): Promise<Metrics> => {
  const config = getConfig();
  const { chain, proverUrl } = createContext(config);

  const prover = await deployProver({
    proverSpec: bench.spec,
  });

  const vlayer = createVlayerClient({
    url: proverUrl,
  });

  let out_metrics: Metrics | undefined = undefined;

  const callback = ({ metrics }: ProofReceipt) => {
    out_metrics = metrics;
  };

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

  if (out_metrics === undefined) {
    throw Error(`no metrics available from benchamrk ${bench.name}`);
  }

  return out_metrics;
};

let benchmarks = [noopBenchmark, ...noopWithCalldataBenchmarks];
let allMetrics: Metrics[] = [];

for (const bench of benchmarks) {
  allMetrics.push(await runBenchmark(bench));
}

console.log("Benchmark results:");

for (let i in benchmarks) {
  const name = benchmarks[i].name;
  const stats = allMetrics[i];
  console.log(`


      ==============  ${name}  ========================
      Gas:  ${stats.gas}
      Cycles:  ${stats.cycles}
      ===============================================


  `);
}
