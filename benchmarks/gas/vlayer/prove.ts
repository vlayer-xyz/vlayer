import { mean, median, sampleStandardDeviation } from "simple-statistics";
import { createVlayerClient, ProofReceipt, Metrics } from "@vlayer/sdk";
import nftSpec from "../out/ExampleNFT.sol/ExampleNFT";
import tokenSpec from "../out/ExampleToken.sol/ExampleToken";
import { isAddress } from "viem";
import {
  getConfig,
  createContext,
  deployVlayerContracts,
  waitForContractDeploy,
} from "@vlayer/sdk/config";

import proverSpec from "../out/SimpleProver.sol/SimpleProver";
import verifierSpec from "../out/SimpleVerifier.sol/SimpleVerifier";

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
const {
  chain,
  ethClient,
  account: john,
  proverUrl,
  confirmations,
} = createContext(config);

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

const nftDeployTransactionHash = await ethClient.deployContract({
  abi: nftSpec.abi,
  bytecode: nftSpec.bytecode.object,
  account: john,
  args: [],
});

const nftContractAddress = await waitForContractDeploy({
  hash: nftDeployTransactionHash,
});

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [tokenAddress],
  verifierArgs: [nftContractAddress],
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
  const [proof, owner, balance] = result;

  if (!isAddress(owner)) {
    throw new Error(`${owner} is not a valid address`);
  }

  const verificationHash = await ethClient.writeContract({
    address: verifier,
    abi: verifierSpec.abi,
    functionName: "claimWhale",
    args: [proof, owner, balance],
    account: john,
  });

  const receipt = await ethClient.waitForTransactionReceipt({
    hash: verificationHash,
    confirmations,
    retryCount: 60,
    retryDelay: 1000,
  });
  if (receipt.status !== "success") {
    throw Error("Verification failed!");
  }

  console.log("  ...success");
}

const stats = out_metrics.toStats();

console.log("Metrics for running simple example: " + JSON.stringify(stats));
