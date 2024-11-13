import { createVlayerClient } from "@vlayer/sdk";
import nftSpec from "../out/ExampleNFT.sol/ExampleNFT";
import tokenSpec from "../out/ExampleToken.sol/ExampleToken";
import { isAddress } from "viem";
import {
  getConfig,
  createContext,
  deployVlayerContracts,
  waitForContractDeploy,
<<<<<<< HEAD
=======
  waitForTransactionReceipt,
>>>>>>> c7756e26 (Use new approach to config in all examples ... (#1108))
} from "@vlayer/sdk/config";

import proverSpec from "../out/SimpleProver.sol/SimpleProver";
import verifierSpec from "../out/SimpleVerifier.sol/SimpleVerifier";

const config = getConfig();
<<<<<<< HEAD
const {
  chain,
  ethClient,
  account: john,
  proverUrl,
  confirmations,
} = await createContext(config);
=======
const { ethClient, account: john } = await createContext(config);
>>>>>>> c7756e26 (Use new approach to config in all examples ... (#1108))

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

const currentBlockNumber = await ethClient.getBlockNumber();

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [tokenAddress, currentBlockNumber],
  verifierArgs: [nftContractAddress],
});

console.log("Proving...");
<<<<<<< HEAD
const vlayer = createVlayerClient({
  url: proverUrl,
});
=======
const vlayer = createVlayerClient();
>>>>>>> c7756e26 (Use new approach to config in all examples ... (#1108))

const hash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "balance",
  args: [john.address],
<<<<<<< HEAD
  chainId: chain.id,
=======
>>>>>>> c7756e26 (Use new approach to config in all examples ... (#1108))
});
const result = await vlayer.waitForProvingResult(hash);
const [proof, owner, balance] = result;

if (!isAddress(owner)) {
  throw new Error(`${owner} is not a valid address`);
}

console.log("Proof result:", result);

const verificationHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "claimWhale",
  args: [proof, owner, balance],
  account: john,
});

<<<<<<< HEAD
const receipt = await ethClient.waitForTransactionReceipt({
  hash: verificationHash,
  confirmations,
  retryCount: 60,
  retryDelay: 1000,
=======
const receipt = await waitForTransactionReceipt({
  hash: verificationHash,
>>>>>>> c7756e26 (Use new approach to config in all examples ... (#1108))
});

console.log(`Verification result: ${receipt.status}`);
