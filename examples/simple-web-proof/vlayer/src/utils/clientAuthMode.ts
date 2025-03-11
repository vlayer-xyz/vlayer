import { privateKeyToAccount } from "viem/accounts";

export enum ClientAuthMode {
  ENV_PRIVATE_KEY = "envPrivateKey",
  WALLET = "wallet",
}

export const useEnvPrivateKey = () => {
  const authMode = import.meta.env.VITE_CLIENT_AUTH_MODE;

  switch (authMode) {
    case ClientAuthMode.ENV_PRIVATE_KEY: {
      if (!import.meta.env.VITE_PRIVATE_KEY) {
        throw new Error("No private key found");
      }
      return true;
    }
    case ClientAuthMode.WALLET: {
      return false;
    }
    default: {
      throw new Error("Invalid VITE_CLIENT_AUTH_MODE: " + authMode);
    }
  }
};

export const getAddressFromPrivateKey = () => {
  let address = "";
  const envPrivateKey = import.meta.env.VITE_PRIVATE_KEY;
  if (!envPrivateKey) {
    throw new Error("No private key found");
  } else {
    address = privateKeyToAccount(envPrivateKey as "0x").address;
  }
  return address as "0x";
};
