import { createVlayerClient } from "@vlayer/sdk";
import proverSpec from "../out/WebProofProver.sol/WebProofProver";
import verifierSpec from "../out/WebProofVerifier.sol/WebProofVerifier";
import web_proof from "../testdata/0.1.0-alpha.8/web_proof.json";
import web_proof_invalid_signature from "../testdata/0.1.0-alpha.8/web_proof_invalid_notary_pub_key.json";
import * as assert from "assert";
import { encodePacked, keccak256 } from "viem";

import {
  getConfig,
  createContext,
  deployVlayerContracts,
  writeEnvVariables,
} from "@vlayer/sdk/config";

let config = getConfig();

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
});

await writeEnvVariables(".env", {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
});

config = getConfig();
const { chain, ethClient, account, proverUrl, confirmations } =
  createContext(config);

if (!account) {
  throw new Error(
    "No account found make sure EXAMPLES_TEST_PRIVATE_KEY is set in your environment variables",
  );
}

const twitterUserAddress = account.address;
const vlayer = createVlayerClient({
  url: proverUrl,
  token: config.token,
});

await testSuccessProvingAndVerification({
  chain,
  ethClient,
  account,
  confirmations,
});

await testFailedProving({ chain });

async function testSuccessProvingAndVerification({
  chain,
  ethClient,
  account,
  confirmations,
}: Required<
  Pick<
    ReturnType<typeof createContext>,
    "chain" | "ethClient" | "account" | "confirmations"
  >
>) {
  console.log("Proving...");

  const hash = await vlayer.prove({
    address: prover,
    functionName: "main",
    proverAbi: proverSpec.abi,
    args: [
      {
        webProofJson: JSON.stringify(web_proof),
      },
      twitterUserAddress,
    ],
    chainId: chain.id,
  });
  const result = await vlayer.waitForProvingResult({ hash });
  const [proof, twitterHandle, address] = result;

  console.log("Verifying...");

  const txHash = await ethClient.writeContract({
    address: verifier,
    abi: verifierSpec.abi,
    functionName: "verify",
    args: [proof, twitterHandle, address],
    chain,
    account,
  });

  await ethClient.waitForTransactionReceipt({
    hash: txHash,
    confirmations,
    retryCount: 60,
    retryDelay: 1000,
  });

  console.log("Verified!");

  const balance = await ethClient.readContract({
    address: verifier,
    abi: verifierSpec.abi,
    functionName: "balanceOf",
    args: [twitterUserAddress],
  });

  assert.strictEqual(balance, 1n);

  const tokenOwnerAddress = await ethClient.readContract({
    address: verifier,
    abi: verifierSpec.abi,
    functionName: "ownerOf",
    args: [generateTokenId(twitterHandle)],
  });

  assert.strictEqual(twitterUserAddress, tokenOwnerAddress);

  const tokenURI = await ethClient.readContract({
    address: verifier,
    abi: verifierSpec.abi,
    functionName: "tokenURI",
    args: [generateTokenId(twitterHandle)],
  });

  assert.strictEqual(
    tokenURI,
    `https://faucet.vlayer.xyz/api/xBadgeMeta?handle=${twitterHandle}`,
  );
}

async function testFailedProving({
  chain,
}: Pick<ReturnType<typeof createContext>, "chain">) {
  try {
    const hash = await vlayer.prove({
      address: prover,
      functionName: "main",
      proverAbi: proverSpec.abi,
      args: [
        {
          webProofJson: JSON.stringify(web_proof_invalid_signature),
        },
        twitterUserAddress,
      ],
      chainId: chain.id,
    });
    await vlayer.waitForProvingResult({ hash });
    throw new Error("Proving should have failed!");
  } catch (error) {
    assert.ok(
      error instanceof Error,
      `Invalid error returned: ${error as string}`,
    );
    assert.equal(
      error.message,
      'Preflight failed with error: Preflight error: Execution error: EVM transact error: revert: ContractError(Revert(Revert("Invalid notary public key")))',
      `Error with wrong message returned: ${error.message}`,
    );
    console.log("✅ Done");
  }
}

function generateTokenId(username: string): bigint {
  return BigInt(keccak256(encodePacked(["string"], [username])));
}
