import { WebProof, createVlayerClient, VlayerClient } from "@vlayer/sdk";

import {
  createExtensionWebProofProvider,
  expectUrl,
  notarize,
  startPage,
} from "@vlayer/sdk/web_proof";

import { foundry } from "viem/chains";
import React, { useCallback, useRef, useState } from "react";
import unconditionalProver from "../../../contracts/out/UnconditionalProver.sol/UnconditionalProver";

const PROVER_ADDRESS = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

function Source() {
  const [proof, setProof] = useState<WebProof>();
  const vlayerClient = useRef<VlayerClient>();

  const requestWebProof = useCallback(async () => {
    const provider = createExtensionWebProofProvider({
      notaryUrl: "http://localhost:7047",
      wsProxyUrl: "ws://localhost:55688",
    });
    const loginUrl = `${window.location.origin}${import.meta.env.BASE_URL}login`;
    const targetUrl = `${window.location.origin}${import.meta.env.BASE_URL}target`;

    vlayerClient.current = createVlayerClient({
      webProofProvider: provider,
    }) as VlayerClient;

    const webproof = await provider.getWebProof({
      //@ts-expect-error this part is not implemented yet in our tlsn flow
      proverCallCommitment: {},
      steps: [
        startPage(loginUrl, "Go to login"),
        expectUrl(targetUrl, "Logged in and appear at target page"),
        notarize("https://swapi.dev/api/people/1", "GET", "Prove"),
      ],
    });

    setProof(webproof);
  }, []);

  const requestZkProof = useCallback(async () => {
    const zkProof = await vlayerClient.current?.prove({
      address: PROVER_ADDRESS,
      proverAbi: unconditionalProver.abi,
      functionName: "web_proof",
      chainId: foundry.id,
      args: [
        {
          webProofJson: JSON.stringify(proof || {}),
        },
      ],
    });
    console.log("ZK proof", zkProof);
  }, []);

  const handleWebProofRequestClick = () => {
    requestWebProof().catch((error) => {
      console.error("Error during requesting web proof:", error);
    });
  };

  const handleZkProofRequestClick = () => {
    requestZkProof().catch((error) => {
      console.error("Error during requesting zk proof:", error);
    });
  };

  return (
    <div className="container">
      <div>
        <button
          data-testid="request-webproof-button"
          onClick={handleWebProofRequestClick}
        >
          Request web proof
        </button>
        {proof ? (
          <>
            <h1 data-testid="has-webproof">Has web proof</h1>
            <button
              data-testid="zk-prove-button"
              onClick={handleZkProofRequestClick}
            />
          </>
        ) : (
          <h1> No web proof </h1>
        )}
      </div>
    </div>
  );
}

export default Source;
