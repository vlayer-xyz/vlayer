// result of env parsing
// mostly needed by the examples to be able to properly perform
// pre run deployment

export type DeployConfig = {
  isTesting: boolean;
};

export type EnvConfig = {
  chainName: string;
  proverUrl: string;
  jsonRpcUrl: string;
  l2JsonRpcUrl?: string;
  privateKey: `0x${string}`;
  token?: string;
  deployConfig?: DeployConfig;
};

// represents what is needed by client to properly
// work in whole vlayer flow
// privateKey is optional and used only for anvil
// to avoid involving metamask into the flow

export type VlayerContextConfig = {
  chainName: string;
  jsonRpcUrl: string;
  proverUrl: string;
  wsProxyUrl?: string;
  notaryUrl?: string;
  privateKey?: `0x${string}`;
  deployConfig?: DeployConfig;
};
