import { createVlayerClient } from "@vlayer/sdk";
import nftSpec from "../out/ExampleNFT.sol/ExampleNFT";
import tokenSpec from "../out/ExampleToken.sol/ExampleToken";
import {
  getConfig,
  createContext,
  deployVlayerContracts,
  waitForContractDeploy,
} from "@vlayer/sdk/config";

import proverSpec from "../out/SimpleProver.sol/SimpleProver";
import verifierSpec from "../out/SimpleVerifier.sol/SimpleVerifier";

const config = getConfig();
const {
  chain,
  ethClient,
  account: john,
  proverUrl,
  confirmations,
} = createContext(config);

if (!john) {
  throw new Error(
    "No account found make sure EXAMPLES_TEST_PRIVATE_KEY is set in your environment variables",
  );
}

const INITIAL_TOKEN_SUPPLY = BigInt(10_000_000);

const tokenDeployTransactionHash = await ethClient.deployContract({
  abi: tokenSpec.abi,
  bytecode: tokenSpec.bytecode.object,
  account: john,
  args: [john.address, INITIAL_TOKEN_SUPPLY],
});

const tokenAddress = await waitForContractDeploy({
  client: ethClient,
  hash: tokenDeployTransactionHash,
});

const nftDeployTransactionHash = await ethClient.deployContract({
  abi: nftSpec.abi,
  bytecode: nftSpec.bytecode.object,
  account: john,
  args: [],
});

const nftContractAddress = await waitForContractDeploy({
  client: ethClient,
  hash: nftDeployTransactionHash,
});

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [tokenAddress],
  verifierArgs: [nftContractAddress],
});

console.log("Proving...");
const vlayer = createVlayerClient({
  url: proverUrl,
  token: config.token,
});

const hash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "balance",
  args: [john.address],
  chainId: chain.id,
});
const result = await vlayer.waitForProvingResult({ hash });
const [proof, owner, balance] = result;

console.log("Proof result:", result);

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

console.log(`Verification result: ${receipt.status}`);
