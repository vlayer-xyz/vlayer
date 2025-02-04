import dotenvflow from "dotenv-flow";
import { type DeployConfig, type EnvConfig } from "./types";

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

const POSSIBLE_VLAYER_ENVS = ["testnet", "dev"] as const;
type VlayerEnv = (typeof POSSIBLE_VLAYER_ENVS)[number];

const ensureVlayerEnv = (): VlayerEnv => {
  try {
    if (!process.env.VLAYER_ENV) {
      throw new Error(
        `VLAYER_ENV is not set. Available options: ${POSSIBLE_VLAYER_ENVS.join(", ")}`,
      );
    }
    if (!POSSIBLE_VLAYER_ENVS.includes(process.env.VLAYER_ENV as VlayerEnv)) {
      throw new Error(
        `Invalid VLAYER_ENV: ${process.env.VLAYER_ENV}. Available options: ${POSSIBLE_VLAYER_ENVS.join(", ")}`,
      );
    }
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
  } catch (e) {
    console.error(e);
    return "dev";
  }

  return process.env.VLAYER_ENV as VlayerEnv;
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

const envVars = [
  { var: "CHAIN_NAME" },
  { var: "PROVER_URL" },
  { var: "JSON_RPC_URL" },
  { var: "L2_JSON_RPC_URL", optional: true },
  { var: "EXAMPLES_TEST_PRIVATE_KEY", to: "privateKey" },
  { var: "VLAYER_API_TOKEN", to: "token", optional: true },
];

export const getConfig = () => {
  dotEnvFlowConfig();
  console.log(ensureVlayerEnv());
  const deployConfig: DeployConfig = {
    isTesting: true,
  };

  return envVars.reduce(
    (config, envVar) => {
      try {
        return {
          ...config,
          [envVar.to ?? toCamelCase(envVar.var)]: ensureEnvVariable(envVar.var),
        };
      } catch (e) {
        if (envVar.optional) {
          return { ...config };
        }
        throw e;
      }
    },
    { deployConfig } as EnvConfig,
  );
};
