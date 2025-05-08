import { describe, test, expect } from "vitest";
import { getConfig } from "./getConfig";

describe("getConfig", () => {
  function mockEnv(env: Record<string, string>) {
    const originalEnv = process.env;
    process.env = env;
    return () => {
      process.env = originalEnv;
    };
  }

  test("correctly parses the environment variables", () => {
    const cleanup = mockEnv({
      VLAYER_ENV: "dev",
      CHAIN_NAME: "ethereum",
      PROVER_URL: "http://localhost:3000",
      JSON_RPC_URL: "http://localhost:8545",
      EXAMPLES_TEST_PRIVATE_KEY:
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
      VLAYER_API_TOKEN: "token",
      SHOULD_DEPLOY_VERIFIER_ROUTER: "true",
      GAS_LIMIT: "1000",
    });

    const config = getConfig();

    expect(config).toEqual({
      vlayerEnv: "dev",
      chainName: "ethereum",
      proverUrl: "http://localhost:3000",
      jsonRpcUrl: "http://localhost:8545",
      privateKey:
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
      token: "token",
      deployConfig: {
        shouldRedeployVerifierRouter: true,
      },
      gasLimit: 1000,
    });

    cleanup();
  });

  test("correctly parses the environment variables with override", () => {
    const cleanup = mockEnv({
      VLAYER_ENV: "dev",
      CHAIN_NAME: "ethereum",
      PROVER_URL: "http://localhost:3000",
      JSON_RPC_URL: "http://localhost:8545",
      EXAMPLES_TEST_PRIVATE_KEY:
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
      VLAYER_API_TOKEN: "token",
      SHOULD_DEPLOY_VERIFIER_ROUTER: "true",
      GAS_LIMIT: "1000",
    });

    const config = getConfig({
      chainName: "polygon",
      deployConfig: {
        shouldRedeployVerifierRouter: false,
      },
      gasLimit: 1_000_000,
    });

    expect(config).toEqual({
      vlayerEnv: "dev",
      chainName: "polygon",
      proverUrl: "http://localhost:3000",
      jsonRpcUrl: "http://localhost:8545",
      privateKey:
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
      token: "token",
      deployConfig: {
        shouldRedeployVerifierRouter: false,
      },
      gasLimit: 1_000_000,
    });

    cleanup();
  });
});
