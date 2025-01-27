import { useEffect } from "react";
import {
  useCallProver,
  useWaitForProvingResult,
  useWebProof,
} from "@vlayer/react";
import { useLocalStorage } from "usehooks-ts";
import { GetWebProofArgs, ProveArgs } from "@vlayer/sdk";
import { Abi, ContractFunctionName } from "viem";
import { optimismSepolia, anvil } from "viem/chains";
import { startPage, expectUrl, notarize } from "@vlayer/sdk/web_proof";

import webProofProver from "../../../out/WebProofProver.sol/WebProofProver";

const vlayerProverConfig: Omit<
  ProveArgs<Abi, ContractFunctionName<Abi>>,
  "args"
> = {
  address: import.meta.env.VITE_PROVER_ADDRESS as `0x${string}`,
  proverAbi: webProofProver.abi,
  chainId:
    import.meta.env.VITE_CHAIN_NAME === "anvil" ? anvil.id : optimismSepolia.id,
  functionName: "main",
  token: import.meta.env.VITE_VLAYER_API_TOKEN,
};

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
    startPage("https://x.com", "Go to x.com login page"),
    expectUrl("https://x.com/home", "Log in"),
    notarize(
      "https://api.x.com/1.1/account/settings.json",
      "GET",
      "Generate Proof of Twitter profile",
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

export const useSimpleWebProof = () => {
  const {
    requestWebProof,
    webProof,
    isPending: isWebProofPending,
  } = useWebProof(webProofConfig);

  const {
    callProver,
    isPending: isCallProverPending,
    data: hash,
  } = useCallProver(vlayerProverConfig);

  const { isPending: isWaitingForProvingResult, data: result } =
    useWaitForProvingResult(hash);

  const [, setWebProof] = useLocalStorage("webProof", "");
  const [, setProverResult] = useLocalStorage("proverResult", "");

  useEffect(() => {
    if (webProof) {
      console.log("webProof ready", webProof);
      setWebProof(JSON.stringify(webProof));
    }
  }, [webProof]);

  useEffect(() => {
    if (result) {
      console.log("proverResult", result);
      setProverResult(JSON.stringify(result));
    }
  }, [result]);

  return {
    requestWebProof,
    webProof,
    isPending:
      isWebProofPending || isCallProverPending || isWaitingForProvingResult,
    callProver,
    result,
  };
};
