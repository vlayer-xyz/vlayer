import {
  createExtensionWebProofProvider,
  expectUrl,
  notarize,
  startPage,
} from "@vlayer/sdk";
import React from "react";
import "./App.css";

const requestWebProof = async () => {
  const provider = createExtensionWebProofProvider({});
  await provider.getWebProof({
    //@ts-expect-error this part is not impelmented yet in our tlsn flow
    proverCallCommitment: {},
    steps: [startPage("", ""), expectUrl("", ""), notarize("", "", "")],
  });
};

function App() {
  return (
    <button
      data-testid="request-webproof-button"
      onClick={requestWebProof}
    ></button>
  );
}

export default App;
