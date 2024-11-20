import { createVlayerClient, VlayerClient } from "@vlayer/sdk";

import {
  createExtensionWebProofProvider,
  createWebProofPlaceholder,
  expectUrl,
  notarize,
  startPage,
} from "@vlayer/sdk/web_proof";

import { foundry } from "viem/chains";
import React, { useCallback, useRef, useState } from "react";
import unconditionalProver from "../../../contracts/fixtures/out/UnconditionalProver.sol/UnconditionalProver";

const PROVER_ADDRESS = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
const NOTARY_PUB_KEY =
  "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEBv36FI4ZFszJa0DQFJ3wWCXvVLFr\ncRzMG5kaTeHGoSzDu6cFqx3uEWYpFGo6C0EOUgf+mEgbktLrXocv5yHzKg==\n-----END PUBLIC KEY-----";

function Source() {
  const vlayerClient = useRef<VlayerClient>();

  const requestProveWeb = useCallback(async () => {
    const provider = createExtensionWebProofProvider({
      notaryUrl: "http://localhost:7047",
      wsProxyUrl: "ws://localhost:55688",
    });
    const loginUrl = `${window.location.origin}${import.meta.env.BASE_URL}login`;
    const targetUrl = `${window.location.origin}${import.meta.env.BASE_URL}target`;
    const webProofPlaceholder = createWebProofPlaceholder({
      logoUrl: "",
      steps: [
        startPage(loginUrl, "Go to login"),
        expectUrl(targetUrl, "Logged in and appear at target page"),
        notarize("https://swapi.dev/api/people/1", "GET", "Prove"),
      ],
      notaryPubKey: NOTARY_PUB_KEY,
      });

    vlayerClient.current = createVlayerClient({
      webProofProvider: provider,
    });

    const zkProof = await vlayerClient.current?.proveWeb({
      address: PROVER_ADDRESS,
      proverAbi: unconditionalProver.abi,
      functionName: "web_proof",
      chainId: foundry.id,
      args: [webProofPlaceholder],
    });

    console.log("ZK proof", zkProof);
  }, []);

  const handleProveWebRequestClick = () => {
    requestProveWeb().catch((error) => {
      console.error("Error during requesting prove Web:", error);
    });
  };

  return (
    <div className="container">
      <div>
        <button
          data-testid="request-prove-web-button"
          onClick={handleProveWebRequestClick}
        >
          Request proof
        </button>
      </div>
    </div>
  );
}

export default Source;
