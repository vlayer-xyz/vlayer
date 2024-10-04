import webProofProver from "../../out/WebProofProver.sol/WebProofProver";

import { foundry } from "viem/chains";

import {
  createExtensionWebProofProvider,
  expectUrl,
  notarize,
  startPage,
  prove,
  type WebProof,
  type VCallResponse,
} from "@vlayer/sdk";
import { createTestClient, http, publicActions, walletActions } from "viem";
import webProofVerifier from "../../out/WebProofVerifier.sol/WebProofVerifier";
import { testHelpers } from "@vlayer/sdk";

const context: {
  webProof: WebProof | null;
  zkProof: VCallResponse | null;
  result: `0x${string}`[];
} = {
  webProof: null,
  zkProof: null,
  result: [],
};

const twitterUserAddress = (await testHelpers.getTestAddresses())[0];

export async function setupRequestProveButton(element: HTMLButtonElement) {
  element.addEventListener("click", async () => {
    const provider = createExtensionWebProofProvider({});
    context.webProof = await provider.getWebProof({
      proverCallCommitment: {
        address: import.meta.env.VITE_PROVER_ADDRESS,
        proverAbi: webProofProver.abi,
        chainId: foundry.id,
        functionName: "main",
        commitmentArgs: [],
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

    const { proof, returnValue } = await prove(
      import.meta.env.VITE_PROVER_ADDRESS,
      webProofProver.abi,
      "main",
      [
        {
          webProofJson: JSON.stringify(webProof),
        },
        twitterUserAddress,
      ],
    );

    context.zkProof = proof;
    context.result = returnValue;
  });
};

export const setupVerifyButton = (element: HTMLButtonElement) => {
  element.addEventListener("click", async () => {
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
    console.log("Verified!", verification);
  });
};
