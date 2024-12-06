import {
  PresentationJSON,
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
import unconditionalProver from "../../../contracts/fixtures/out/UnconditionalProver.sol/UnconditionalProver";

const PROVER_ADDRESS = import.meta.env.VITE_UNCONDITIONAL_PROVER_ADDRESS;
console.log(PROVER_ADDRESS);

function SourceNewWay() {
  const [proof, setProof] = useState<PresentationJSON | null>(null);

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
      ({ payload: { proof } }) => {
        setProof(proof);
      },
    );
  }, []);

  const requestWebProof = useCallback(() => {
    const loginUrl = `${window.location.origin}${import.meta.env.BASE_URL}login`;
    const targetUrl = `${window.location.origin}${import.meta.env.BASE_URL}target`;

    webProofProvider.requestWebProof({
      proverCallCommitment: {
        address: PROVER_ADDRESS,
        proverAbi: unconditionalProver.abi,
        chainId: foundry.id,
        functionName: "web_proof",
        commitmentArgs: [],
      },
      logoUrl: "",
      steps: [
        startPage(loginUrl, "Go to login"),
        expectUrl(targetUrl, "Logged in and appear at target page"),
        notarize("https://lotr-api.online:3011/regular_json", "GET", "Prove"),
      ],
    });
  }, []);

  const requestZkProof = useCallback(async () => {
    const zkProof = await vlayerClient.prove({
      address: PROVER_ADDRESS,
      proverAbi: unconditionalProver.abi,
      functionName: "web_proof",
      chainId: foundry.id,
      args: [
        {
          webProofJson: JSON.stringify({
            presentation_json: proof,
          }),
        },
      ],
    });
    console.log("ZK proof", zkProof);
  }, [proof]);

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

function Source() {
  const [proof, setProof] = useState<PresentationJSON>();
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
    });

    const webproof = await provider.getWebProof({
      proverCallCommitment: {
        address: PROVER_ADDRESS,
        proverAbi: unconditionalProver.abi,
        chainId: foundry.id,
        functionName: "web_proof",
        commitmentArgs: [],
      },
      logoUrl: "",
      steps: [
        startPage(loginUrl, "Go to login"),
        expectUrl(targetUrl, "Logged in and appear at target page"),
        notarize("https://lotr-api.online:3011/regular_json", "GET", "Prove"),
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
          webProofJson: JSON.stringify({
            presentation_json: proof,
          }),
        },
      ],
    });
    console.log("ZK proof", zkProof);
  }, [proof]);

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

export { Source, SourceNewWay };
