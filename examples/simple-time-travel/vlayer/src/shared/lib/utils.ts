export const shortenAndFormatHash = (hash: string | null) =>
  hash ? `${hash.slice(0, 4)}...${hash.slice(-4)}` : ""; // 0x00...012 instead of long hash
