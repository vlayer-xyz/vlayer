import { createVlayerClient } from "@vlayer/sdk";

import {
  createExtensionWebProofProvider,
  createWebProofRequest,
  expectUrl,
  notarize,
  startPage,
} from "@vlayer/sdk/web_proof";

import { foundry } from "viem/chains";
import React, { useMemo, useState, useCallback } from "react";
import lotrApiProver from "../../../contracts/fixtures/out/LotrApiProver.sol/LotrApiProver";

const PROVER_ADDRESS = import.meta.env
  .VITE_LOTR_API_PROVER_ADDRESS as `0x${string}`;

function DappProveWeb() {
  const [zkProof, setZkProof] = useState<boolean>();
  const [name, setName] = useState<string>();
  const [greeting, setGreeting] = useState<string>();

  const webProofProvider = useMemo(() => {
    return createExtensionWebProofProvider({
      notaryUrl: "http://localhost:7047",
      wsProxyUrl: "ws://localhost:3003",
    });
  }, []);

  const vlayerClient = useMemo(() => {
    return createVlayerClient({
      webProofProvider,
    });
  }, [webProofProvider]);

  const requestZkProof = useCallback(async () => {
    const loginUrl = `${window.location.origin}${import.meta.env.BASE_URL}login`;
    const profileUrl = `${window.location.origin}${import.meta.env.BASE_URL}profile`;
    const dashboardUrl = `${window.location.origin}${import.meta.env.BASE_URL}dashboard`;

    const webProofRequest = createWebProofRequest({
      logoUrl: "",
      steps: [
        startPage(loginUrl, "Go to login"),
        expectUrl(dashboardUrl, "At dashboard page"),
        expectUrl(profileUrl, "At profile page"),
        notarize(
          "https://lotr-api.online:3011/regular_json?are_you_sure=yes*",
          "GET",
          "Prove",
          [
            {
              response: {
                json_body_except: ["name"],
              },
            },
            {
              response: {
                headers: [
                  "AcceSs-COntrol-ExposE-Headers",
                  "Access-Control-Allow-Headers",
                ],
              },
            },
            {
              request: {
                url_query: ["really"],
              },
            },
            {
              request: {
                headers: ["connection"],
              },
            },
          ],
        ),
      ],
    });

    const hash = await vlayerClient.proveWeb({
      address: PROVER_ADDRESS,
      proverAbi: lotrApiProver.abi,
      chainId: foundry.id,
      functionName: "web_proof",
      args: [webProofRequest],
    });

    const zkProof = await vlayerClient.waitForProvingResult({ hash });
    const name = zkProof[1];
    const greeting = zkProof[2];
    setName(name);
    setGreeting(greeting);
    setZkProof(zkProof);
    setZkProof(zkProof);
  }, []);

  const handleZkProofRequestClick = () => {
    requestZkProof().catch((error) => {
      console.error("Error during requesting zk proof:", error);
    });
  };

  return (
    <div className="container">
      <div>
        <button onClick={handleZkProofRequestClick}>Request prove web</button>
      </div>
      <div>
        {zkProof ? (
          <div>
            <h1>Has zk proof</h1>
            <h2>Name from prover:</h2>
            <pre
              style={{ whiteSpace: "break-spaces" }}
              data-testid="name-from-prover"
            >
              {name}
            </pre>
            <h2>Greeting from prover:</h2>
            <pre
              style={{ whiteSpace: "break-spaces" }}
              data-testid="greeting-from-prover"
            >
              {greeting}
            </pre>
          </div>
        ) : (
          <h1> No zk proof </h1>
        )}
      </div>
    </div>
  );
}

export { DappProveWeb };
