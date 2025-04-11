//#entry point to flow where we use PUT request
import {
  useWebProof,
  expectUrl,
  WebProofConfig,
  notarize,
  ProofProvider,
  startPage,
} from "@vlayer/react";

import React from "react";
import { Abi } from "viem";

const webProofConfig: WebProofConfig<Abi, string> = {
  proverCallCommitment: {
    address: "0x0000000000000000000000000000000000000000",
    proverAbi: [],
    functionName: "proveWeb",
    commitmentArgs: [],
    chainId: 1,
  },
  logoUrl: "http://twitterswap.com/logo.png",
  steps: [
    startPage(
      `${window.location.origin}${import.meta.env.BASE_URL}login`,
      "Go to login page",
    ),
    expectUrl(
      `${window.location.origin}${import.meta.env.BASE_URL}dashboard`,
      "At dashboard page",
    ),
    notarize(
      "https://lotr-api.online:3011/update_resource",
      "PUT",
      "Generate Proof of Update Resource",
      [
        {
          request: {
            // redact all the headers
            headers_except: [],
          },
        },
      ],
    ),
  ],
};

function DappPutContent() {
  const { requestWebProof, webProof } = useWebProof(webProofConfig);

  return (
    <div
      style={{ display: "flex", flexDirection: "column", alignItems: "center" }}
    >
      <button onClick={requestWebProof}>Request Web Proof</button>
      {webProof && <h1>Has web proof</h1>}
    </div>
  );
}

function DappPut() {
  return (
    <ProofProvider
      config={{
        notaryUrl: "http://localhost:7047",
        wsProxyUrl: "ws://localhost:3003",
      }}
    >
      <DappPutContent />
    </ProofProvider>
  );
}

export { DappPut };
