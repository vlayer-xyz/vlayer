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
    startPage("https://x.com", "Go to X"),
    expectUrl("https://x.com/home", "Log in"),
    notarize(
      "https://x.com/i/api/graphql/*/Followers",
      "GET",
      "Generate Proof of Followers",
    ),
  ],
};

export const useSimpleWebProof = () => {
  const {
    requestWebProof,
    webProof,
    decodedTranscript,
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
  const [, setDecodedTranscript] = useLocalStorage("decodedTranscript", "");
  useEffect(() => {
    if (webProof) {
      console.log("webProof ready", webProof);
      setWebProof(JSON.stringify(webProof));
    }
  }, [webProof]);

  useEffect(() => {
    if (decodedTranscript) {
      setDecodedTranscript(JSON.stringify(decodedTranscript));
    }
  }, [decodedTranscript]);

  useEffect(() => {
    if (result) {
      console.log("proverResult", result);
      setProverResult(JSON.stringify(result));
    }
  }, [result]);

  return {
    decodedTranscript,
    requestWebProof,
    webProof,
    isPending:
      isWebProofPending || isCallProverPending || isWaitingForProvingResult,
    callProver,
    result,
  };
};
