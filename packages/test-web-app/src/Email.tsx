import { createVlayerClient, preverifyEmail } from "@vlayer/sdk";
import proverSpec from "../../../contracts/fixtures/out/EmailProver.sol/EmailProver";
import React, { useCallback, ChangeEvent } from "react";
import { foundry } from "viem/chains";

const PROVER_ADDRESS = import.meta.env.VITE_EMAIL_PROVER_ADDRESS;
const getStrFromFile = (file: File): Promise<string> => {
  const reader = new FileReader();

  return new Promise((resolve, reject) => {
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = () => reject(new Error("Reader error"));
    reader.readAsText(file);
  });
};

const useEmailFileUpload = () => {
  return useCallback(async (evt: ChangeEvent<HTMLInputElement>) => {
    const file = evt.target.files?.[0];
    if (!file) {
      return;
    }
    const vlayer = createVlayerClient();
    const str = await getStrFromFile(file);
    const unverifiedEmail = await preverifyEmail(str);
    const x = await vlayer.prove({
      address: PROVER_ADDRESS,
      proverAbi: proverSpec.abi,
      functionName: "main",
      chainId: foundry.id,
      args: [unverifiedEmail],
    });
    console.log("c", x);
  }, []);
};

export default function Email() {
  const handleFileChange = useEmailFileUpload();
  return (
    <>
      <h1>Email</h1>
      <input
        name="file"
        type="file"
        onChange={(evt) => {
          console.log("File changed");
          void handleFileChange(evt);
        }}
      />
    </>
  );
}
