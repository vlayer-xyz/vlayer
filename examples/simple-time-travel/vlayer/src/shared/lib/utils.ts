export const usePrivateKey =
  import.meta.env.VITE_USE_WINDOW_ETHEREUM_TRANSPORT === "false";

export const truncateHashOrAddr = (hash: string | null) =>
  hash ? `${hash.slice(0, 4)}...${hash.slice(-4)}` : "";
