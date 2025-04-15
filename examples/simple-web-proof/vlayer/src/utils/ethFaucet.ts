import { type Address } from "viem";
import { createPublicClient, http, parseEther } from "viem";
import { optimismSepolia } from "viem/chains";
import { FaucetError } from "../errors";
import { z } from "zod";

export const publicClient = createPublicClient({
  chain: optimismSepolia,
  transport: http(),
});

export const ensureBalance = async (address: Address, balance: bigint) => {
  console.log("ensureBalance", address, balance);
  if (balance > parseEther("0.00002")) {
    return;
  }

  console.log("not enough balance, funding needed");

  if (!import.meta.env.VITE_FAUCET_URL) {
    console.warn("no faucet url, skipping funding account");
    return;
  }

  const response = await fetch(
    `${import.meta.env.VITE_FAUCET_URL}/faucet?address=${address}`,
    {
      method: "POST",
    },
  );

  if (!response.ok) {
    throw new FaucetError();
  }

  const responseSchema = z.object({
    transactionHash: z
      .string()
      .regex(/^0x[0-9a-fA-F]+$/, "Must be a hex string starting with 0x"),
  });

  const parsedResponse = responseSchema.parse(await response.json()) as {
    transactionHash: `0x${string}`;
  };

  const { transactionHash: hash } = parsedResponse;
  console.log("waiting for tx to be confirmed", hash);

  await publicClient.waitForTransactionReceipt({ hash });

  return hash;
};
