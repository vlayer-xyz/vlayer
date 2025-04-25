export const shortenAndFormatHash = (hash: string | null) =>
  hash ? `${hash.slice(0, 4)}...${hash.slice(-4)}` : ""; // 0x00...012 instead of long hash

const chainIdToName = {
  "1": "Ethereum Mainnet",
  "10": "Optimism Mainnet",
  "8453": "Base Mainnet",
  "84532": "Base Sepolia",
  "11155111": "Ethereum Sepolia",
  "11155420": "OP Sepolia",
  "31338": "Anvil#2",
  "31337": "Anvil#1",
} as const;

export const getChainName = (chainId: string): string => {
  return (
    chainIdToName[chainId as keyof typeof chainIdToName] || `Chain ${chainId}`
  );
};

export const tokensToProve = JSON.parse(
  import.meta.env.VITE_TOKENS_TO_CHECK,
) as {
  addr: string;
  chainId: string;
  blockNumber: string;
  balance: string;
}[];

export const parseProverResult = (proverResult: string) =>
  JSON.parse(proverResult) as [
    unknown,
    `0x${string}`,
    { addr: string; chainId: string; blockNumber: string; balance: string }[],
  ];
