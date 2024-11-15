import dotenvflow from "dotenv-flow";

export type Config = {
  chainName: string;
  proverUrl: string;
  jsonRpcUrl: string;
  privateKey: `0x${string}`;
};

const ensureEnvVariable = (envVar: string) => {
  if (!process.env[envVar]) {
    if (envVar === "EXAMPLES_TEST_PRIVATE_KEY") {
      throw new Error(
        `${envVar} missing. Add a HEX private key with ETH in .env.local for deploy and verify transactions.`,
      );
    }
    throw new Error(`${envVar} is not set`);
  }
  return process.env[envVar];
};

const ensureVlayerEnv = () => {
  try {
    if (!process.env.VLAYER_ENV) {
      throw new Error("VLAYER_ENV is not set. Available options: testnet, dev");
    }
    if (!["testnet", "dev"].includes(process.env.VLAYER_ENV)) {
      throw new Error(
        `Invalid VLAYER_ENV: ${process.env.VLAYER_ENV}. Available options: testnet, anvil, mainnet`,
      );
    }
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
  } catch (e) {
    return "dev";
  }

  return process.env.VLAYER_ENV;
};

const dotEnvFlowConfig = () => {
  dotenvflow.config({
    node_env: ensureVlayerEnv(),
  });
};

export const toCamelCase = (str: string) =>
  str
    .toLowerCase()
    .replace(/([-_][a-z])/g, (group) =>
      group.toUpperCase().replace("-", "").replace("_", ""),
    );

const requiredEnvVars = [
  { var: "CHAIN_NAME" },
  { var: "PROVER_URL" },
  { var: "JSON_RPC_URL" },
  { var: "EXAMPLES_TEST_PRIVATE_KEY", to: "privateKey" },
];

export const getConfig = () => {
  dotEnvFlowConfig();
  return requiredEnvVars.reduce((config, envVar) => {
    return {
      ...config,
      [envVar.to ?? toCamelCase(envVar.var)]: ensureEnvVariable(envVar.var),
    };
  }, {} as Config);
};
