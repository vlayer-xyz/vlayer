import webProofProver from "../../out/WebProofProver.sol/WebProofProver";

import { foundry, optimismSepolia } from "viem/chains";

import {
  createExtensionWebProofProvider,
  expectUrl,
  notarize,
  startPage,
  createVlayerClient,
  type WebProof,
  testHelpers,
  type Proof,
  isDefined,
} from "@vlayer/sdk";
import webProofVerifier from "../../out/WebProofVerifier.sol/WebProofVerifier";
import { Hex, createWalletClient, http } from "viem";
import { privateKeyToAccount } from "viem/accounts";

console.log("Hello from VLayer!");

const context: {
  webProof: WebProof | undefined;
  provingResult: [Proof, string, Hex] | undefined;
} = { webProof: undefined, provingResult: undefined };

const twitterUserAddress = (await testHelpers.getTestAddresses())[0];

export async function setupRequestProveButton(element: HTMLButtonElement) {
  element.addEventListener("click", async () => {
    const provider = createExtensionWebProofProvider();
    const webProof = await provider.getWebProof({
      proverCallCommitment: {
        address: import.meta.env.VITE_PROVER_ADDRESS,
        proverAbi: webProofProver.abi,
        chainId: Number(import.meta.env.VITE_CHAIN_ID) || foundry.id,
        functionName: "main",
        commitmentArgs: ["0x"],
      },
      logoUrl: "http://twitterswap.com/logo.png",
      steps: [
        startPage("https://x.com/i/flow/login", "Go to x.com login page"),
        expectUrl("https://x.com/home", "Log in"),
        notarize(
          "https://api.x.com/1.1/account/settings.json",
          "GET",
          "Generate Proof of Twitter profile",
        ),
      ],
    });

    console.log("WebProof generated!", webProof);
    context.webProof = webProof;
  });
}

export const setupVProverButton = (element: HTMLButtonElement) => {
  element.addEventListener("click", async () => {
    const notaryPubKey =
      "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

    const webProof = {
      tls_proof: context.webProof,
      notary_pub_key: notaryPubKey,
    };
    const vlayer = createVlayerClient();

    console.log("Generating proof...");
    const { hash } = await vlayer.prove({
      address: import.meta.env.VITE_PROVER_ADDRESS,
      functionName: "main",
      proverAbi: webProofProver.abi,
      args: [
        {
          webProofJson: JSON.stringify(webProof),
        },
        twitterUserAddress,
      ],
      chainId: Number(import.meta.env.VITE_CHAIN_ID) || foundry.id,
    });
    const provingResult = await vlayer.waitForProvingResult({
      hash,
    });
    console.log("Proof generated!", provingResult);
    context.provingResult = provingResult as [Proof, string, Hex];
  });
};

export const setupVerifyButton = (element: HTMLButtonElement) => {
  element.addEventListener("click", async () => {
    isDefined(context.provingResult, "Proving result is undefined");

    if (import.meta.env.VITE_CHAIN_ID && import.meta.env.VITE_TEST_PRIV_KEY) {
      const walletClient = createWalletClient({
        chain: optimismSepolia,
        transport: http("https://sepolia.optimism.io"),
      });

      const deployer = privateKeyToAccount(
        import.meta.env.VITE_TEST_PRIV_KEY as `0x${string}`,
      );

      const txHash = await walletClient.writeContract({
        address: import.meta.env.VITE_VERIFIER_ADDRESS,
        abi: webProofVerifier.abi,
        functionName: "verify",
        args: context.provingResult,
        account: deployer,
      });
      console.log(
        `Verification tx: https://sepolia-optimism.etherscan.io/tx/${txHash}`,
      );
    } else {
      const verification = await testHelpers.createAnvilClient().writeContract({
        address: import.meta.env.VITE_VERIFIER_ADDRESS,
        abi: webProofVerifier.abi,
        functionName: "verify",
        args: context.provingResult,
        account: twitterUserAddress,
        chain: undefined,
      });
      console.log("Verified!", verification);
    }
  });
};
