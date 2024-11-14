import { useState, useEffect } from "react";
import { Address } from "viem";
import * as chains from "viem/chains";
import useProver from "../hooks/useProver";
import { preverifyEmail } from "@vlayer/sdk";
import { getStrFromFile } from "../lib/utils";
import proverSpec from "../../../../out/EmailDomainProver.sol/EmailDomainProver";
import verifierSpec from "../../../../out/EmailProofVerifier.sol/EmailDomainVerifier";

import EmlForm from "../components/EmlForm";
import { createContext, customTransport, type Chain } from "@vlayer/sdk/config";

declare global {
  interface Window {
    ethereum: { request: () => Promise<unknown> };
  }
}

function getChainByName(name: string) {
  const chain = (chains as Record<string, Chain>)[name];
  if (chain) {
    return chain;
  } else {
    throw new Error("Chain does not exist");
  }
}

const { ethClient: walletClient } = await createContext(
  {
    chainName: import.meta.env.VITE_CHAIN_NAME as string,
    proverUrl: import.meta.env.VITE_PROVER_URL as string,
    jsonRpcUrl: import.meta.env.VITE_JSON_RPC_URL as string,
    privateKey: import.meta.env.VITE_PRIVATE_KEY as `0x${string}`,
  },
  import.meta.env.VITE_USE_WINDOW_ETHEREUM_TRANSPORT
    ? customTransport(window.ethereum)
    : undefined,
);

const EmlUploadForm = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [currentStep, setCurrentStep] = useState("");
  const [errorMsg, setErrorMsg] = useState("");
  const [successMsg, setSuccessMsg] = useState("");
  const [claimerAddr, setClaimerAddr] = useState<Address>("0x");
  const chain = getChainByName(import.meta.env.VITE_CHAIN_NAME as string);

  const { prove, proof, provingError } = useProver({
    addr: import.meta.env.VITE_PROVER_ADDRESS as Address,
    abi: proverSpec.abi,
    func: "main",
    chainId: chain.id,
  });

  const getClaimerAddr = async () => {
    if (typeof window !== "undefined" && !window.ethereum)
      throw new Error("no_wallet_detected");

    if (chain.name !== chains.anvil.name) {
      await walletClient.switchChain({ id: chain.id });
    }
    const [addr] = await walletClient.requestAddresses();

    setClaimerAddr(addr);
    return addr;
  };

  const manageError = (err: unknown) => {
    console.log({ err });
    setIsSubmitting(false);
    if (err instanceof Error) {
      setErrorMsg(err.message);
    } else {
      setErrorMsg("Something went wrong, check logs");
    }
  };

  const verifyProof = async () => {
    try {
      setCurrentStep("Verifying on-chain...");

      if (proof == null) throw new Error("no_proof_to_verify");

      const txHash = await walletClient.writeContract({
        address: import.meta.env.VITE_VERIFIER_ADDRESS as `0x${string}`,
        abi: verifierSpec.abi,
        functionName: "verify",
        args: proof,
        chain,
        account: claimerAddr,
      });

      const receipt = await walletClient.waitForTransactionReceipt({
        hash: txHash,
      });
      console.log("receipt", receipt);
      setCurrentStep("Success!");
      setIsSubmitting(false);
      if (chain.blockExplorers && receipt.status === "success") {
        window.open(`${chain.blockExplorers?.default.url}/tx/${txHash}`);
      } else if (receipt.status === "reverted") {
        setErrorMsg("Transaction reverted. Is email already used?");
        window.open(`${chain.blockExplorers?.default.url}/tx/${txHash}`);
      } else {
        setSuccessMsg("Verified successfully.");
      }
    } catch (err) {
      manageError(err);
    }
  };

  const startProving = async (uploadedEmlFile: File, claimerAddr: Address) => {
    setCurrentStep("Sending to prover...");

    const eml = await getStrFromFile(uploadedEmlFile);
    const email = await preverifyEmail(eml);
    await prove([email, claimerAddr]);
    setCurrentStep("Waiting for proof...");
  };

  const submit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsSubmitting(true);
    setErrorMsg("");

    const formData = new FormData(e.currentTarget);
    const emlFile = formData.get("emlFile") as File | null;
    if (!emlFile) throw new Error("no_eml_file");

    setCurrentStep("Connecting to wallet...");
    const addr = await getClaimerAddr();
    await startProving(emlFile, addr);
    setCurrentStep("Waiting for proof...");
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    submit(e).catch((err) => {
      manageError(err);
    });
  };

  useEffect(() => {
    if (proof) {
      verifyProof().catch((err) => {
        manageError(err);
      });
    }
  }, [proof]);

  useEffect(() => {
    if (provingError) {
      setErrorMsg(provingError);
      setIsSubmitting(false);
    }
  }, [provingError]);

  return (
    <EmlForm
      isSubmitting={isSubmitting}
      handleSubmit={handleSubmit}
      errorMsg={errorMsg}
      successMsg={successMsg}
      currentStep={currentStep}
    />
  );
};

export default EmlUploadForm;
