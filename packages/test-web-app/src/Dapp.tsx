import {
  type PresentationJSON,
  type VGetProofReceiptResult,
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

const PROVER_ADDRESS = import.meta.env
  .VITE_UNCONDITIONAL_PROVER_ADDRESS as `0x${string}`;
console.log(PROVER_ADDRESS);

function DappNewWay() {
  const [webProof, setWebProof] = useState<PresentationJSON>();
  const [zkProof, setZkProof] = useState<boolean>();

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
        setWebProof(proof);
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
        proverAbi: unconditionalProver.abi,
        chainId: foundry.id,
        functionName: "web_proof",
        commitmentArgs: [],
      },
      logoUrl: "",
      steps: [
        startPage(loginUrl, "Go to login"),
        expectUrl(dashboardUrl, "At dashboard page"),
        expectUrl(profileUrl, "At profile page"),
        notarize("https://lotr-api.online:3011/regular_json", "GET", "Prove"),
      ],
    });
  }, []);

  const requestZkProof = useCallback(async () => {
    const hash = await vlayerClient.prove({
      address: PROVER_ADDRESS,
      proverAbi: unconditionalProver.abi,
      functionName: "web_proof",
      chainId: foundry.id,
      args: [
        {
          webProofJson: JSON.stringify({
            presentation_json: webProof,
          }),
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
        Request proof of beeing wizzard
      </button>

      <div>
        {webProof ? (
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
        expectUrl(dashboardUrl, "At dashboard page"),
        expectUrl(profileUrl, "At profile page"),
        notarize("https://lotr-api.online:3011/regular_json", "GET", "Prove"),
      ],
    });

    setWebProof(webproof);
  }, []);

  const requestZkProof = useCallback(async () => {
    const hash = await vlayerClient.current?.prove({
      address: PROVER_ADDRESS,
      proverAbi: unconditionalProver.abi,
      functionName: "web_proof",
      chainId: foundry.id,
      args: [
        {
          webProofJson: JSON.stringify({
            presentation_json: webProof,
          }),
        },
      ],
    });
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
        Request proof of beeing wizzard
      </button>

      <div>
        {webProof ? (
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
