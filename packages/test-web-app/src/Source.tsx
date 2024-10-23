import {
  createExtensionWebProofProvider,
  expectUrl,
  notarize,
  startPage,
} from "@vlayer/sdk";
import React, { useCallback, useEffect, useState } from "react";

function Source() {
  const [hasProof, setHasProof] = useState(true);
  const [proof, setProof] = useState<unknown>(null);
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
    console.log("webproof", webproof);
    setProof(webproof);
  }, []);

  useEffect(() => {
    setHasProof(!!proof);
  }, [proof]);
  return (
    <>
      <button data-testid="request-webproof-button" onClick={requestWebProof}>
        Request web proof
      </button>
      {hasProof ? (
        <h1 data-testid="has-webproof">Has web proof</h1>
      ) : (
        <h1> No web proof </h1>
      )}
    </>
  );
}

export default Source;
