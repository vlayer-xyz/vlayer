export const shortenAndFormatHash = (hash: string | null) =>
  hash ? `${hash.slice(0, 4)}...${hash.slice(-4)}` : ""; // 0x00...012 instead of long hash

const chainIdToName = {
  "11155420": "Sepolia",
  "31338": "Anvil#2",
  "31337": "Anvil#1",
} as const;

export const getChainName = (chainId: string): string => {
  return (
    chainIdToName[chainId as keyof typeof chainIdToName] || `Chain ${chainId}`
  );
};
