import {
  type PresentationJSON,
  createVlayerClient,
  MessageFromExtensionType,
} from "@vlayer/sdk";

import {
  createExtensionWebProofProvider,
  expectUrl,
  notarize,
  startPage,
} from "@vlayer/sdk/web_proof";

import { foundry } from "viem/chains";
import React, { useEffect, useMemo, useState, useCallback } from "react";
import lotrApiProver from "../../../contracts/fixtures/out/LotrApiProver.sol/LotrApiProver";

const PROVER_ADDRESS = import.meta.env
  .VITE_LOTR_API_PROVER_ADDRESS as `0x${string}`;

function Dapp() {
  return DappWithProfile("profile");
}

function DappFailedAuth() {
  return DappWithProfile("profile-failed-auth");
}

function DappWithProfile(profile: string) {
  const [webProof, setWebProof] = useState<PresentationJSON>();
  const [zkProof, setZkProof] = useState<boolean>();
  const [name, setName] = useState<string>();
  const [greeting, setGreeting] = useState<string>();

  const [decodedResponse, setDecodedResponse] = useState<string>();
  const [decodedRequest, setDecodedRequest] = useState<string>();

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

  useEffect(() => {
    webProofProvider.addEventListeners(
      MessageFromExtensionType.ProofDone,
      ({
        payload,
      }: {
        payload: {
          presentationJson: PresentationJSON;
          decodedTranscript: { sent: string; recv: string };
        };
      }) => {
        setWebProof(payload.presentationJson);
        setDecodedResponse(payload.decodedTranscript.recv);
        setDecodedRequest(payload.decodedTranscript.sent);
      },
    );
  }, []);

  const requestWebProof = useCallback(() => {
    const loginUrl = `${window.location.origin}${import.meta.env.BASE_URL}login`;
    const profileUrl = `${window.location.origin}${import.meta.env.BASE_URL}${profile}`;
    const dashboardUrl = `${window.location.origin}${import.meta.env.BASE_URL}dashboard`;
    webProofProvider.requestWebProof({
      proverCallCommitment: {
        address: PROVER_ADDRESS,
        proverAbi: lotrApiProver.abi,
        chainId: foundry.id,
        functionName: "web_proof",
        commitmentArgs: [],
      },
      logoUrl: "",
      steps: [
        startPage(loginUrl, "Go to login page"),
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
  }, []);

  const requestZkProof = useCallback(async () => {
    const hash = await vlayerClient.prove({
      address: PROVER_ADDRESS,
      proverAbi: lotrApiProver.abi,
      functionName: "web_proof",
      chainId: foundry.id,
      args: [
        {
          webProofJson: JSON.stringify({ presentationJson: webProof }),
        },
      ],
    });
    const zkProof = await vlayerClient.waitForProvingResult({ hash });
    const name = zkProof[1];
    const greeting = zkProof[2];
    setName(name);
    setGreeting(greeting);
    setZkProof(zkProof);
  }, [webProof]);

  const handleWebProofRequestClick = () => {
    requestWebProof();
  };

  const handleZkProofRequestClick = () => {
    requestZkProof().catch((error) => {
      console.error("Error during requesting zk proof:", error);
    });
  };

  return (
    <div className="container">
      <button onClick={handleWebProofRequestClick}>
        Request proof of being a wizard
      </button>

      <div>
        {webProof ? (
          <>
            <h1>Has web proof</h1>
            <button onClick={handleZkProofRequestClick}>
              Request zk proof
            </button>
          </>
        ) : (
          <h1> No web proof </h1>
        )}
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
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "10px",
          marginTop: "20px",
        }}
      >
        {decodedRequest && (
          <div>
            <h2>Request:</h2>
            <pre
              style={{ whiteSpace: "break-spaces" }}
              data-testid="redacted-request"
            >
              {decodedRequest}
            </pre>
          </div>
        )}
        {decodedResponse && (
          <div>
            <h2>Response:</h2>
            <pre
              style={{ whiteSpace: "break-spaces" }}
              data-testid="redacted-response"
            >
              {decodedResponse}
            </pre>
          </div>
        )}
      </div>
    </div>
  );
}

export { Dapp, DappFailedAuth };
