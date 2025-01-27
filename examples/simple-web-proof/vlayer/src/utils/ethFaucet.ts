import { type Address } from "viem";
import { createPublicClient, http, parseEther } from "viem";
import { optimismSepolia } from "viem/chains";

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
    console.error("failed to fund account", response);
    return;
  }

  const { transactionHash: hash } = await response.json();
  console.log("waiting for tx to be confirmed", hash);

  await publicClient.waitForTransactionReceipt({ hash });

  return hash;
};
