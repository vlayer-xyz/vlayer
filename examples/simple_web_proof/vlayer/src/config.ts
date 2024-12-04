import proverSpec from "../../out/WebProofProver.sol/WebProofProver.json";
import verifierSpec from "../../out/WebProofVerifier.sol/WebProofVerifier.json";
import { expectUrl, notarize, startPage } from "@vlayer/sdk/web_proof";
import { createContext } from "@vlayer/sdk/config";
import * as chains from "viem/chains";
import { type Abi, type Hex } from "viem";

const { account } = await createContext({
  chainName: import.meta.env.VITE_CHAIN_NAME,
  proverUrl: import.meta.env.VITE_PROVER_URL,
  jsonRpcUrl: import.meta.env.VITE_JSON_RPC_URL,
  privateKey: import.meta.env.VITE_PRIVATE_KEY,
});

export const config = {
  proverCallCommitment: {
    address: import.meta.env.VITE_PROVER_ADDRESS as Hex,
    proverAbi: proverSpec.abi as Abi,
    chainId: chains[import.meta.env.VITE_CHAIN_NAME as keyof typeof chains].id,
    functionName: "main",
    commitmentArgs: ["0x"],
  },
  logoUrl: "http://twitterswap.com/logo.png",
  steps: [
    startPage("https://x.com/i/flow/login", "Go to x.com login page"),
    expectUrl("https://x.com/home", "Log in"),
    notarize(
      "https://api.x.com/1.1/account/settings.json",
      "GET",
      "Generate Proof of Twitter profile",
    ),
  ],
  account,
  verifierAbi: verifierSpec.abi,
  notaryPubKey:
    "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n",
};
