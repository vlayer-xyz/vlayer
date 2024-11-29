const PROVER_ADDRESS = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
import proverSpec from "../../out/WebProofProver.sol/WebProofProver";
import verifierSpec from "../../out/WebProofVerifier.sol/WebProofVerifier";
import { expectUrl, notarize, startPage } from "@vlayer/sdk/web_proof";
import { createContext } from "@vlayer/sdk/config";
import * as chains from "viem/chains";

const { chain, ethClient, account, proverUrl, confirmations } =
  await createContext({
    chainName: import.meta.env.VITE_CHAIN_NAME,
    proverUrl: import.meta.env.VITE_PROVER_URL,
    jsonRpcUrl: import.meta.env.VITE_JSON_RPC_URL,
    privateKey: import.meta.env.VITE_PRIVATE_KEY,
  });

export const config = {
  proverCallCommitment: {
    address: PROVER_ADDRESS as `0x${string}`,
    proverAbi: proverSpec.abi,
    chainId: chains[import.meta.env.VITE_CHAIN_NAME as keyof typeof chains].id,
    functionName: "web_proof",
    commitmentArgs: [] as [],
  },
  logoUrl: "",
  steps: [
    startPage("https://demo.tink.com/", "Go to tink"),
    expectUrl("https://demo.tink.com/account-check", "Go to account check"),
    notarize(
      "https://demo.tink.com/api/report?*",
      "GET",
      "Prove",
      "userDataByProvider.0.accounts.0.accountNumber",
    ),
  ],
  account,
  verifierAbi: verifierSpec.abi,
  notaryPubKey:
    "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n",
};
