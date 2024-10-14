import {
  createExtensionWebProofProvider,
  expectUrl,
  notarize,
  startPage,
} from "@vlayer/sdk";
import React from "react";

const requestWebProof = async () => {
  const provider = createExtensionWebProofProvider({});
  const loginUrl = `${window.location.origin}${import.meta.env.BASE_URL}login`;
  const targetUrl = `${window.location.origin}${import.meta.env.BASE_URL}target`;

  await provider.getWebProof({
    //@ts-expect-error this part is not impelmented yet in our tlsn flow
    proverCallCommitment: {},
    steps: [
      startPage(loginUrl, "Go to login"),
      expectUrl(targetUrl, "Logged in and appear at target page"),
      notarize("https://swapi.dev/api/people/1", "GET", "Prove"),
    ],
  });
};

function Source() {
  return (
    <button data-testid="request-webproof-button" onClick={requestWebProof}>
      Request web proof
    </button>
  );
}

export default Source;
