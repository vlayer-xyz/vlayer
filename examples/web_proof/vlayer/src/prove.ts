import { extensionConnector } from "./extensionConnector";
import { prove } from "@vlayer/sdk";
import webProofProver from "../../out/WebProofProver.sol/WebProofProver.json";
import webProofVerifier from "../../out/WebProofVerifier.sol/WebProofVerifier.json";
import { createTestClient, http, publicActions, walletActions } from "viem";
import { foundry } from "viem/chains";

export function setupRequestProveButton(element: HTMLButtonElement) {
  element.addEventListener("click", () => {
    console.log("Requesting proof", import.meta.env.VITE_EXTENSION_ID);
    chrome.runtime.sendMessage(import.meta.env.VITE_EXTENSION_ID, {
      action: "open_side_panel",
    });
  });
}

export const setupVProverButton = (element: HTMLButtonElement) => {
  element.addEventListener("click", async () => {
    const notaryPubKey =
      "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

    console.log("extensionConnector.proof", extensionConnector);
    const webProof = {
      tls_proof: extensionConnector.tlsproof,
      notary_pub_key: notaryPubKey,
    };
    console.log("webProof", webProof);
    console.log("notaryPubKey", notaryPubKey);
    console.log("prover address", import.meta.env.VITE_PROVER_ADDRESS);

    const { proof, returnValue } = await prove(
      import.meta.env.VITE_PROVER_ADDRESS,
      // @ts-expect-error problem with abi
      webProofProver.abi,
      "main",
      [
        {
          webProofJson: JSON.stringify(webProof),
        },
      ],
    );
    console.log("Proof:", proof);
    console.log("returnValue:", returnValue);
    extensionConnector.zkproof = proof;
    extensionConnector.returnValue = returnValue;
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
        args: [extensionConnector.zkproof, extensionConnector.returnValue],
        account: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
      });
    console.log("Verified!", verification);
  });
};
