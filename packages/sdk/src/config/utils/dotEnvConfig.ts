import dotenvflow from "dotenv-flow";

export const dotEnvFlowConfig = () => {
  dotenvflow.config({
    node_env: process.env.VLAYER_ENV,
    default_node_env: "dev",
  });
};
