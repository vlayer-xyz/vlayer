import { mean, sampleStandardDeviation } from "simple-statistics";
import { createVlayerClient, ProofReceipt } from "@vlayer/sdk";
import tokenSpec from "../out/ExampleToken.sol/ExampleToken";
import { isAddress } from "viem";
import {
  getConfig,
  createContext,
  deployProver,
  waitForContractDeploy,
} from "@vlayer/sdk/config";

import proverSpec from "../out/SimpleProver.sol/SimpleProver";

class MetricsUnpacked {
  gas: Array<number> = [];
  cycles: Array<number> = [];
  times: {
    preflight: Array<number>;
    proving: Array<number>;
  } = {
    preflight: [],
    proving: [],
  };

  toStats(): MetricsStats {
    return {
      gas: meanStddev(this.gas),
      cycles: meanStddev(this.cycles),
      times: {
        preflight: meanStddev(this.times.preflight),
        proving: meanStddev(this.times.proving),
      },
    };
  }
}

function meanStddev(values: Array<number>): MeanStddev {
  return {
    mean: mean(values),
    stddev: sampleStandardDeviation(values),
  };
}

type MeanStddev = {
  mean: number;
  stddev: number;
};

type MetricsStats = {
  gas: MeanStddev;
  cycles: MeanStddev;
  times: {
    preflight: MeanStddev;
    proving: MeanStddev;
  };
};

const config = getConfig();
const { chain, ethClient, account: john, proverUrl } = createContext(config);

const INITIAL_TOKEN_SUPPLY = BigInt(10_000_000);

const tokenDeployTransactionHash = await ethClient.deployContract({
  abi: tokenSpec.abi,
  bytecode: tokenSpec.bytecode.object,
  account: john,
  args: [john.address, INITIAL_TOKEN_SUPPLY],
});

const tokenAddress = await waitForContractDeploy({
  hash: tokenDeployTransactionHash,
});

const prover = await deployProver({ proverSpec, proverArgs: [tokenAddress] });

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
    functionName: "balance",
    args: [john.address],
    chainId: chain.id,
    token: config.token,
  });

  const result = await vlayer.waitForProvingResult({
    hash,
    callback,
  });

  if (!isAddress(result[1])) {
    throw new Error(`${result.owner} is not a valid address`);
  }

  console.log("  ...success");
}

const stats = out_metrics.toStats();

console.log("Metrics for running simple example: " + JSON.stringify(stats));
