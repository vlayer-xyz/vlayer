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

  test("correctly parses the environment variables", async () => {
    const cleanup = mockEnv({
      VLAYER_ENV: "dev",
      CHAIN_NAME: "ethereum",
      PROVER_URL: "http://localhost:3000",
      JSON_RPC_URL: "http://localhost:8545",
      EXAMPLES_TEST_PRIVATE_KEY:
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
      VLAYER_API_TOKEN: "token",
      SHOULD_DEPLOY_VERIFIER_ROUTER: "true",
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
    });

    cleanup();
  });

  test("correctly parses the environment variables with override", async () => {
    const cleanup = mockEnv({
      VLAYER_ENV: "dev",
      CHAIN_NAME: "ethereum",
      PROVER_URL: "http://localhost:3000",
      JSON_RPC_URL: "http://localhost:8545",
      EXAMPLES_TEST_PRIVATE_KEY:
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
      VLAYER_API_TOKEN: "token",
      SHOULD_DEPLOY_VERIFIER_ROUTER: "true",
    });

    const config = getConfig({
      chainName: "polygon",
      deployConfig: {
        shouldRedeployVerifierRouter: false,
      },
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
    });

    cleanup();
  });
});
