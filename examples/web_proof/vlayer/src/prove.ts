import webProofProver from "../../out/WebProofProver.sol/WebProofProver";

import { foundry } from "viem/chains";

import {
  asAddress,
  createExtensionWebProofProvider,
  createVlayerClient,
  expectUrl,
  handleAsyncError,
  isDefined,
  notarize,
  type Proof,
  startPage,
  testHelpers,
  type WebProof,
} from "@vlayer/sdk";
import webProofVerifier from "../../out/WebProofVerifier.sol/WebProofVerifier";
import { Address, Hex } from "viem";

console.log("Hello from VLayer!");

const context: {
  webProof?: WebProof;
  provingResult?: [Proof, string, Hex];
} = { webProof: undefined, provingResult: undefined };

const twitterUserAddress = (await testHelpers.getTestAddresses())[0];
const viteVerifierAddress: Address = asAddress(
  import.meta.env.VITE_VERIFIER_ADDRESS,
);

export function setupRequestProveButton(element: HTMLButtonElement) {
  element.addEventListener(
    "click",
    handleAsyncError(async () => {
      const provider = createExtensionWebProofProvider();
      const webProof = await provider.getWebProof({
        proverCallCommitment: {
          address: viteVerifierAddress,
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

      console.log("WebProof generated!", webProof);
      context.webProof = webProof;
    }, "Error during request prove"),
  );
}

export const setupVProverButton = (element: HTMLButtonElement) => {
  element.addEventListener(
    "click",
    handleAsyncError(async () => {
      const notaryPubKey =
        "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

      const webProof = {
        tls_proof: context.webProof,
        notary_pub_key: notaryPubKey,
      };
      const vlayer = createVlayerClient();

      console.log("Generating proof...");
      const { hash } = await vlayer.prove({
        address: viteVerifierAddress,
        functionName: "main",
        proverAbi: webProofProver.abi,
        args: [
          {
            webProofJson: JSON.stringify(webProof),
          },
          twitterUserAddress,
        ],
        chainId: foundry.id,
      });
      const provingResult = await vlayer.waitForProvingResult({
        hash,
      });
      console.log("Proof generated!", provingResult);
      context.provingResult = provingResult as [Proof, string, Hex];
    }, "Error during proving"),
  );
};

export const setupVerifyButton = (element: HTMLButtonElement) => {
  element.addEventListener(
    "click",
    handleAsyncError(async () => {
      isDefined(context.provingResult, "Proving result is undefined");

      const verification = await testHelpers.createAnvilClient().writeContract({
        address: viteVerifierAddress,
        abi: webProofVerifier.abi,
        functionName: "verify",
        args: context.provingResult,
        account: twitterUserAddress,
        chain: undefined,
      });
      console.log("Verified!", verification);
    }, "Error during verification"),
  );
};
