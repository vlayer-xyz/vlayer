import { GetWebProofArgs } from "@vlayer/sdk";
import { Abi } from "viem";

import { startPage, expectUrl, notarize } from "@vlayer/sdk/web_proof";

export const webProofConfig: GetWebProofArgs<Abi, string> = {
  proverCallCommitment: {
    address: "0x0000000000000000000000000000000000000000",
    proverAbi: [],
    functionName: "proveWeb",
    commitmentArgs: [],
    chainId: 1,
  },
  logoUrl: "http://twitterswap.com/logo.png",
  steps: [
    startPage("https://x.com", "Go to x.com login page"),
    expectUrl("https://x.com/home", "Log in"),
    notarize(
      "https://api.x.com/1.1/account/settings.json",
      "GET",
      "Generate Proof of Twitter profile",
    ),
  ],
};
