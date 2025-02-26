export const getChainConfirmations = (chainName?: string): number => {
  if (!chainName || chainName.toLowerCase() === "anvil") {
    return 1;
  }
  return 6;
};
