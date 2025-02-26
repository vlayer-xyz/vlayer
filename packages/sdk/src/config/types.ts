export type DeployConfig = {
  shouldRedeployVerifierRouter?: boolean;
};

export type VlayerContextConfig = {
  chainName: string;
  proverUrl: string;
  jsonRpcUrl: string;
  l2JsonRpcUrl?: string;
  dnsServiceUrl?: string;
  privateKey: `0x${string}`;
  token?: string;
  deployConfig: DeployConfig;
  vlayerEnv: string;
  notaryUrl?: string;
  wsProxyUrl?: string;
};
