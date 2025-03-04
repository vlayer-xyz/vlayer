import { useAppKitAccount } from "@reown/appkit/react";
import { privateKeyToAccount } from "viem/accounts";

export const useAddress = () => {
  let address = "";
  let error: string | null = null;

  const authMode = import.meta.env.VITE_CLIENT_AUTH_MODE;

  try {
    if (authMode == "envPrivateKey") {
      const envPrivateKey = import.meta.env.VITE_PRIVATE_KEY;
      if (!envPrivateKey) {
        throw new Error("No private key found");
      } else {
        address = privateKeyToAccount(envPrivateKey as "0x").address;
      }
    } else if (authMode == "wallet") {
      const addressFromWallet = useAppKitAccount().address;
      if (addressFromWallet) {
        address = addressFromWallet;
      } else {
        throw new Error("No address found in wallet");
      }
    } else {
      throw new Error("Invalid VITE_CLIENT_AUTH_MODE");
    }
  } catch (e) {
    error = (e as Error).message;
  }

  return { address, error };
};
