import {
  type PresentationJSON,
  createVlayerClient,
  type VlayerClient,
  ExtensionMessageType,
} from "@vlayer/sdk";

import {
  createExtensionWebProofProvider,
  expectUrl,
  notarize,
  startPage,
} from "@vlayer/sdk/web_proof";

import { foundry } from "viem/chains";
import React, {
  useEffect,
  useMemo,
  useRef,
  useState,
  useCallback,
} from "react";
import lotrApiProver from "../../../contracts/fixtures/out/LotrApiProver.sol/LotrApiProver";

const PROVER_ADDRESS = import.meta.env
  .VITE_LOTR_API_PROVER_ADDRESS as `0x${string}`;
console.log(PROVER_ADDRESS);

function DappNewWay() {
  const [webProof, setWebProof] = useState<PresentationJSON>();
  const [zkProof, setZkProof] = useState<boolean>();

  const [decodedResponse, setDecodedResponse] = useState<string>();
  const [decodedRequest, setDecodedRequest] = useState<string>();

  const webProofProvider = useMemo(() => {
    return createExtensionWebProofProvider({
      notaryUrl: "http://localhost:7047",
      wsProxyUrl: "ws://localhost:55688",
    });
  }, []);

  const vlayerClient = useMemo(() => {
    return createVlayerClient({
      webProofProvider,
    });
  }, [webProofProvider]);

  useEffect(() => {
    webProofProvider.addEventListeners(
      ExtensionMessageType.ProofDone,
      ({
        payload,
      }: {
        payload: {
          presentationJSON: PresentationJSON;
          decodedTranscript: { sent: string; recv: string };
        };
      }) => {
        setWebProof(payload.presentationJSON);
        setDecodedResponse(payload.decodedTranscript.recv);
        setDecodedRequest(payload.decodedTranscript.sent);
      },
    );
  }, []);

  const requestWebProof = useCallback(() => {
    const loginUrl = `${window.location.origin}${import.meta.env.BASE_URL}login`;
    const profileUrl = `${window.location.origin}${import.meta.env.BASE_URL}profile`;
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
          "https://lotr-api.online:3011/regular_json?are_you_sure=yes",
          "GET",
          "Prove",
          [
            {
              response: {
                json_body_except: ["screen_name"],
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
                url_query: ["are_you_sure"],
              },
            },
            {
              request: {
                headers: ["content-type"],
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
          webProofJson: JSON.stringify(webProof),
        },
      ],
    });
    const zkProof = await vlayerClient.waitForProvingResult({ hash });
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
      <button
        data-testid="request-webproof-button"
        onClick={handleWebProofRequestClick}
      >
        Request proof of beeing a wizard
      </button>

      <div>
        {webProof ? (
          <>
            <h1 data-testid="has-webproof">Has web proof</h1>
            <button
              data-testid="zk-prove-button"
              onClick={handleZkProofRequestClick}
            >
              Request zk proof
            </button>
          </>
        ) : (
          <h1> No web proof </h1>
        )}
      </div>
      <div>
        {zkProof ? (
          <h1 data-testid="has-zkproof">Has zk proof</h1>
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

function Dapp() {
  const [webProof, setWebProof] = useState<PresentationJSON>();
  const [zkProof, setZkProof] = useState<boolean>();
  const vlayerClient = useRef<VlayerClient>();
  const requestWebProof = useCallback(async () => {
    const provider = createExtensionWebProofProvider({
      notaryUrl: "http://localhost:7047",
      wsProxyUrl: "ws://localhost:55688",
    });
    const loginUrl = `${window.location.origin}${import.meta.env.BASE_URL}login`;
    const profileUrl = `${window.location.origin}${import.meta.env.BASE_URL}profile`;
    const dashboardUrl = `${window.location.origin}${import.meta.env.BASE_URL}dashboard`;
    vlayerClient.current = createVlayerClient({
      webProofProvider: provider,
    });

    const webproof: {
      presentationJSON: PresentationJSON;
      decodedTranscript: { sent: string; recv: string };
    } = await provider.getWebProof({
      proverCallCommitment: {
        address: PROVER_ADDRESS,
        proverAbi: lotrApiProver.abi,
        chainId: foundry.id,
        functionName: "web_proof",
        commitmentArgs: [],
      },
      logoUrl: "",
      steps: [
        startPage(loginUrl, "Go to login"),
        expectUrl(dashboardUrl, "At dashboard page"),
        expectUrl(profileUrl, "At profile page"),
        notarize(
          "https://lotr-api.online:3011/regular_json?are_you_sure=yes",
          "GET",
          "Prove",
          [
            {
              response: {
                json_body_except: ["screen_name"],
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
                url_query: ["are_you_sure"],
              },
            },
            {
              request: {
                headers: ["content-type"],
              },
            },
          ],
        ),
      ],
    });

    setWebProof(webproof.presentationJSON);
  }, []);

  const requestZkProof = useCallback(async () => {
    const hash = await vlayerClient.current?.prove({
      address: PROVER_ADDRESS,
      proverAbi: lotrApiProver.abi,
      functionName: "web_proof",
      chainId: foundry.id,
      args: [
        {
          webProofJson: JSON.stringify(webProof),
        },
      ],
    });
    if (!hash) {
      throw new Error("Hash is undefined");
    }

    const zkProof = await vlayerClient.current?.waitForProvingResult({ hash });
    console.log("ZK proof", zkProof);
    setZkProof(zkProof);
  }, [webProof]);

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
      <button
        data-testid="request-webproof-button"
        onClick={handleWebProofRequestClick}
      >
        Request proof of beeing a wizard
      </button>

      <div>
        {webProof ? (
          <>
            <h1 data-testid="has-webproof">Has web proof</h1>
            <button
              data-testid="zk-prove-button"
              onClick={handleZkProofRequestClick}
            >
              Request zk proof
            </button>
          </>
        ) : (
          <h1> No web proof </h1>
        )}
      </div>
      <div>
        {zkProof ? (
          <h1 data-testid="has-zkproof">Has zk proof</h1>
        ) : (
          <h1> No zk proof </h1>
        )}
      </div>
    </div>
  );
}

export { Dapp, DappNewWay };
