import { createVlayerClient, ProofReceipt } from "@vlayer/sdk";
import { getConfig, createContext, deployProver } from "@vlayer/sdk/config";
import { MetricsUnpacked } from "./types";

import proverSpec from "../../out/NoopProver.sol/NoopProver";

const config = getConfig();
const { chain, proverUrl } = createContext(config);

const prover = await deployProver({ proverSpec, proverArgs: [] });

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

for (let i = 0; i < 10; i++) {
  console.log(`Executing ${i} time...`);

  const hash = await vlayer.prove({
    address: prover,
    proverAbi: proverSpec.abi,
    functionName: "noop",
    args: [],
    chainId: chain.id,
    token: config.token,
  });

  await vlayer.waitForProvingResult({
    hash,
    callback,
  });

  console.log("  ...success");
}

const stats = out_metrics.toStats();

console.log("Metrics for running simple example: " + JSON.stringify(stats));
