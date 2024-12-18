import webProofProver from "../../out/WebProofProver.sol/WebProofProver";

import { foundry } from "viem/chains";

import {
  createVlayerClient,
  type PresentationJSON,
  type Proof,
  isDefined,
} from "@vlayer/sdk";

import {
  createWebProofRequest,
  expectUrl,
  notarize,
  startPage,
} from "@vlayer/sdk/web_proof";

import { createContext } from "@vlayer/sdk/config";

import webProofVerifier from "../../out/WebProofVerifier.sol/WebProofVerifier";
import { Hex } from "viem";

const context: {
  webProof: PresentationJSON | undefined;
  provingResult: [Proof, string, Hex] | undefined;
} = { webProof: undefined, provingResult: undefined };

const { chain, ethClient, account, proverUrl, confirmations } = createContext({
  chainName: import.meta.env.VITE_CHAIN_NAME,
  proverUrl: import.meta.env.VITE_PROVER_URL,
  jsonRpcUrl: import.meta.env.VITE_JSON_RPC_URL,
  privateKey: import.meta.env.VITE_PRIVATE_KEY,
});

const twitterUserAddress = account.address;

export const setupProveWebButton = (element: HTMLButtonElement) => {
  element.addEventListener("click", async () => {
    const vlayer = createVlayerClient({
      url: proverUrl,
    });

    const webProofRequest = createWebProofRequest({
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

    const hash = await vlayer.proveWeb({
      address: import.meta.env.VITE_PROVER_ADDRESS,
      proverAbi: webProofProver.abi,
      chainId: foundry.id,
      functionName: "main",
      token: import.meta.env.VITE_VLAYER_API_TOKEN,
      args: [webProofRequest, twitterUserAddress],
    });

    const provingResult = await vlayer.waitForProvingResult({ hash });
    console.log("Proof generated!", provingResult);
    context.provingResult = provingResult as [Proof, string, Hex];
  });
};

export const setupVerifyButton = (element: HTMLButtonElement) => {
  element.addEventListener("click", async () => {
    isDefined(context.provingResult, "Proving result is undefined");

    const txHash = await ethClient.writeContract({
      address: import.meta.env.VITE_VERIFIER_ADDRESS,
      abi: webProofVerifier.abi,
      functionName: "verify",
      args: context.provingResult,
      chain,
      account: account,
    });

    const verification = await ethClient.waitForTransactionReceipt({
      hash: txHash,
      confirmations,
      retryCount: 60,
      retryDelay: 1000,
    });
    console.log("Verified!", verification);
  });
};
