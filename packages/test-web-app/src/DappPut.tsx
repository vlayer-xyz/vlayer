//#etry point to flow where we use post request
import {
  useWebProof,
  expectUrl,
  GetWebProofArgs,
  notarize,
  ProofProvider,
  startPage,
} from "@vlayer/react";

import React, { useEffect } from "react";
import { Abi } from "viem";

const webProofConfig: GetWebProofArgs<Abi, string> = {
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

  useEffect(() => {
    if (webProof) {
      console.log("webProof ready", webProof);
    }
  }, [JSON.stringify(webProof)]);
  return (
    <div>
      <button onClick={requestWebProof}>Request Web Proof</button>
    </div>
  );
}

function DappPut() {
  return (
    <ProofProvider>
      <DappPutContent />
    </ProofProvider>
  );
}

export { DappPut };
