import webProofProver from "../../out/WebProofProver.sol/WebProofProver";

import { foundry } from "viem/chains";

import {
  createExtensionWebProofProvider,
  expectUrl,
  notarize,
  startPage,
  createVlayerClient,
  type WebProof,
  type VCallResponse,
  testHelpers,
} from "@vlayer/sdk";
import webProofVerifier from "../../out/WebProofVerifier.sol/WebProofVerifier";

const context: {
  webProof: WebProof | null;
  zkProof: VCallResponse | null;
  result: `0x${string}`[];
} = {
  webProof: null,
  zkProof: null,
  result: [],
};

export async function setupRequestProveButton(element: HTMLButtonElement) {
  element.addEventListener("click", async () => {
    const provider = createExtensionWebProofProvider();
    const webproof = await provider.getWebProof({
      proverCallCommitment: {
        address: import.meta.env.VITE_PROVER_ADDRESS,
        proverAbi: webProofProver.abi,
        chainId: foundry.id,
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

    context.webProof = webproof;
  });
}

export const setupVProverButton = async (element: HTMLButtonElement) => {
  element.addEventListener("click", async () => {
    const notaryPubKey =
      "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

    const webProof = {
      tls_proof: context.webProof,
      notary_pub_key: notaryPubKey,
    };
    const vlayer = createVlayerClient();

    console.log("Generating proof...");
<<<<<<< HEAD
    const { hash } = await vlayer.prove({
      address: import.meta.env.VITE_PROVER_ADDRESS,
      functionName: "main",
      proverAbi: webProofProver.abi,
      args: [
=======
    const twitterUserAddress = (await testHelpers.getTestAddresses())[0];

    const { proof, returnValue } = await prove(
      import.meta.env.VITE_PROVER_ADDRESS,
      webProofProver.abi,
      "main",
      [
>>>>>>> 43f93efe (Add logic to run headless tests on CI)
        {
          webProofJson: JSON.stringify(webProof),
        },
        twitterUserAddress,
      ],
    });
    const { proof, result } = await vlayer.waitForProvingResult({ hash });
    console.log("Proof generated!", proof, result);
    context.zkProof = proof;
    context.result = result;
  });
};

export const setupVerifyButton = async (element: HTMLButtonElement) => {
  element.addEventListener("click", async () => {
<<<<<<< HEAD
    const verification = await testHelpers.createAnvilClient().writeContract({
      address: import.meta.env.VITE_VERIFIER_ADDRESS,
      abi: webProofVerifier.abi,
      functionName: "verify",
      args: [context.zkProof, ...context.result],
      account: twitterUserAddress,
    });
=======
    const twitterUserAddress = (await testHelpers.getTestAddresses())[0];
    const verification = await createTestClient({
      chain: foundry,
      mode: "anvil",
      transport: http(),
    })
      .extend(publicActions)
      .extend(walletActions)
      .writeContract({
        address: import.meta.env.VITE_VERIFIER_ADDRESS,
        abi: webProofVerifier.abi,
        functionName: "verify",
        args: [context.zkProof, ...context.result],
        account: twitterUserAddress,
      });
>>>>>>> 43f93efe (Add logic to run headless tests on CI)
    console.log("Verified!", verification);
  });
};
