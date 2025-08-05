export const getChainConfirmations = (chainName?: string): number => {
  if (!chainName || chainName.toLowerCase() === "anvil") {
    return 1;
  }
  return 6;
};

export const getChainRetries = (chainName?: string): number => {
  if (!chainName || chainName.toLowerCase() === "anvil") {
    return 60;
  }
  return 600;
};
