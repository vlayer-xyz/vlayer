import {
  createExtensionWebProofProvider,
  expectUrl,
  notarize,
  startPage,
  WebProof,
} from "@vlayer/sdk";
import React, { useCallback, useState } from "react";

function Source() {
  const [proof, setProof] = useState<WebProof>();
  const requestWebProof = useCallback(async () => {
    const provider = createExtensionWebProofProvider({
      notaryUrl: "http://localhost:7047",
      wsProxyUrl: "ws://localhost:55688",
    });
    const loginUrl = `${window.location.origin}${import.meta.env.BASE_URL}login`;
    const targetUrl = `${window.location.origin}${import.meta.env.BASE_URL}target`;

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

  const handleClick = () => {
    requestWebProof().catch((error) => {
      console.error("Error during requesting web proof:", error);
    });
  };

  return (
    <div className="container">
      <div>
        <button data-testid="request-webproof-button" onClick={handleClick}>
          Request web proof
        </button>
        {proof ? (
          <h1 data-testid="has-webproof">Has web proof</h1>
        ) : (
          <h1> No web proof </h1>
        )}
      </div>
    </div>
  );
}

export default Source;
