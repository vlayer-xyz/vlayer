import dotenvflow from "dotenv-flow";

const ensureEnvVariable = (envVar: string) => {
  if (!process.env[envVar]) {
    throw new Error(`${envVar} is not set`);
  }
  return process.env[envVar];
};
const dotEnvFlowConfig = () => {
  dotenvflow.config({
    node_env: ensureEnvVariable("VLAYER_ENV"),
  });
};

const toCamelCase = (str: string) => {
  return str
    .replace(/_./g, (match) => match.charAt(1).toUpperCase())
    .replace(/^./, (match) => match.toLowerCase());
};

const requiredEnvVars = [
  "CHAIN_NAME",
  "PROVER_URL",
  "JSON_RPC_URL",
  "PRIVATE_KEY",
];

export const getConfig = () => {
  dotEnvFlowConfig();
  return requiredEnvVars.reduce(
    (config, envVar) => {
      return { ...config, [toCamelCase(envVar)]: ensureEnvVariable(envVar) };
    },
    {} as Record<string, string>,
  );
};
