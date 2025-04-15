import {
  describe,
  expect,
  it,
  vi,
  beforeEach,
  type MockInstance,
  afterEach,
} from "vitest";

import { createExtensionWebProofProvider } from "../webProof";
import { createVlayerClient } from "./client";
import { type BrandedHash, type VlayerClient } from "types/vlayer";
import { ZkProvingStatus } from "../../web-proof-commons";
import createFetchMock from "vitest-fetch-mock";
import { HttpAuthorizationError, VLAYER_ERROR_NOTES } from "./errors";

declare const global: {
  chrome: object;
};

const fetchMocker = createFetchMock(vi);

beforeEach(() => {
  fetchMocker.enableMocks();
  global.chrome = {
    runtime: {
      sendMessage: vi.fn(),
      connect: vi.fn().mockImplementation(() => {
        return {
          onMessage: {
            addListener: vi.fn(),
          },
          postMessage: vi.fn(),
        };
      }),
    },
  };
});

afterEach(() => {
  fetchMocker.disableMocks();
});

function generateRandomHash() {
  let hash = "0x";
  for (let i = 0; i < 64; ++i) {
    hash += Math.floor(Math.random() * 16).toString(16);
  }
  return hash;
}

describe("Success zk-proving", () => {
  let hashStr: string;
  let zkProvingSpy: MockInstance<(status: ZkProvingStatus) => void>;
  let vlayer: VlayerClient;

  beforeEach(() => {
    hashStr = generateRandomHash();
    const webProofProvider = createExtensionWebProofProvider();
    zkProvingSpy = vi.spyOn(webProofProvider, "notifyZkProvingStatus");
    vlayer = createVlayerClient({ webProofProvider });
  });
  it("should send message to extension that zkproving started", async () => {
    fetchMocker.mockResponseOnce(() => {
      return {
        body: JSON.stringify({
          id: 1,
          jsonrpc: "2.0",
          result: hashStr,
        }),
      };
    });

    const result = await vlayer.prove({
      address: `0x${"a".repeat(40)}`,
      functionName: "main",
      proverAbi: [],
      args: [],
      chainId: 42,
    });

    expect(result.hash).toBe(hashStr);
    expect(zkProvingSpy).toBeCalledTimes(1);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(1, ZkProvingStatus.Proving);
  });
  it("should send message to extension that zkproving is done", async () => {
    fetchMocker.mockResponseOnce(() => {
      return {
        body: JSON.stringify({
          id: 1,
          jsonrpc: "2.0",
          result: hashStr,
        }),
      };
    });

    await vlayer.prove({
      address: `0x${"a".repeat(40)}`,
      functionName: "main",
      proverAbi: [],
      args: [],
      chainId: 42,
    });

    fetchMocker.mockResponseOnce(() => {
      return {
        body: JSON.stringify({
          result: {
            state: "done",
            status: 1,
            metrics: {},
            data: {},
          },
          jsonrpc: "2.0",
          id: 1,
        }),
      };
    });

    const hash = { hash: hashStr } as BrandedHash<[], string>;
    await vlayer.waitForProvingResult({ hash });

    expect(zkProvingSpy).toBeCalledTimes(2);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(1, ZkProvingStatus.Proving);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(2, ZkProvingStatus.Done);
  });
  it("should notify that zk-proving failed", async () => {
    fetchMocker.mockResponseOnce(() => {
      throw new Error("test");
    });

    const hash = { hash: hashStr } as BrandedHash<[], string>;
    try {
      await vlayer.waitForProvingResult({ hash });
    } catch (e) {
      //eslint-disable-next-line no-console
      console.log("Error waiting for proving result", e);
    }

    expect(zkProvingSpy).toBeCalledTimes(1);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(1, ZkProvingStatus.Error);
  });
});

describe("Failed zk-proving", () => {
  let zkProvingSpy: MockInstance<(status: ZkProvingStatus) => void>;
  let vlayer: VlayerClient;
  let hashStr: string;

  beforeEach(() => {
    hashStr = generateRandomHash();
    const webProofProvider = createExtensionWebProofProvider();
    zkProvingSpy = vi.spyOn(webProofProvider, "notifyZkProvingStatus");
    vlayer = createVlayerClient({ webProofProvider });
  });
  it("should send message to extension that zkproving started and then that failed when server error 500", async () => {
    fetchMocker.mockResponseOnce((req) => {
      if (req.url === "http://127.0.0.1:3000/") {
        return {
          status: 500,
        };
      }
      return {};
    });

    try {
      const hash = await vlayer.prove({
        address: `0x${"a".repeat(40)}`,
        functionName: "main",
        proverAbi: [],
        args: [],
        chainId: 42,
      });
      await vlayer.waitForProvingResult({ hash });
    } catch (e) {
      //eslint-disable-next-line no-console
      console.log("Error waiting for proving result", e);
    }

    expect(zkProvingSpy).toBeCalledTimes(2);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(1, ZkProvingStatus.Proving);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(2, ZkProvingStatus.Error);
  });
  it("should send message to extension that zkproving started and then that failed when computation failed at any stage", async () => {
    fetchMocker.mockResponseOnce(() => {
      return {
        body: JSON.stringify({
          id: 1,
          jsonrpc: "2.0",
          result: hashStr,
        }),
      };
    });

    const hash = await vlayer.prove({
      address: `0x${"a".repeat(40)}`,
      functionName: "main",
      proverAbi: [],
      args: [],
      chainId: 42,
    });

    fetchMocker.mockResponseOnce(() => {
      return {
        body: JSON.stringify({
          result: {
            state: "preflight",
            status: 0,
            metrics: {},
            error: "Preflight error: ...",
          },
          jsonrpc: "2.0",
          id: 1,
        }),
      };
    });

    try {
      await vlayer.waitForProvingResult({ hash });
    } catch (e) {
      expect((e as Error).message).toMatch(
        "Preflight failed with error: Preflight error: ...",
      );
    }

    expect(zkProvingSpy).toBeCalledTimes(2);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(1, ZkProvingStatus.Proving);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(2, ZkProvingStatus.Error);
  });
});

describe("Authentication", () => {
  let hashStr: string;

  beforeEach(() => {
    hashStr = generateRandomHash();
  });

  it("requires passing a token", async () => {
    const userToken = "deadbeef";
    const vlayer = createVlayerClient({ token: userToken });

    fetchMocker.mockResponseOnce((req) => {
      const token = (req.headers.get("authorization") || "")
        .split("Bearer ")
        .at(1);
      if (token !== undefined && token === userToken) {
        return {
          body: JSON.stringify({
            id: 1,
            jsonrpc: "2.0",
            result: hashStr,
          }),
        };
      }
      return {
        status: 401,
        body: JSON.stringify({
          error: "Invalid JWT token",
        }),
      };
    });
    const hash = await vlayer.prove({
      address: `0x${"a".repeat(40)}`,
      functionName: "main",
      proverAbi: [],
      args: [],
      chainId: 42,
    });

    fetchMocker.mockResponseOnce((req) => {
      const token = (req.headers.get("authorization") || "")
        .split("Bearer ")
        .at(1);
      if (token !== undefined && token === userToken) {
        return {
          body: JSON.stringify({
            result: {
              state: "done",
              status: 1,
              metrics: {},
              data: {},
            },
            jsonrpc: "2.0",
            id: 1,
          }),
        };
      }
      return {
        status: 401,
        body: JSON.stringify({
          error: "Invalid JWT token",
        }),
      };
    });
    await vlayer.waitForProvingResult({ hash });

    expect(hash.hash).toBe(hashStr);
  });

  describe("fails with a readable error if", () => {
    beforeEach(() => {
      fetchMocker.mockResponseOnce((req) => {
        const token = (req.headers.get("authorization") || "")
          .split("Bearer ")
          .at(1);
        if (token === undefined) {
          return {
            status: 401,
            body: JSON.stringify({
              error: "Missing JWT token",
            }),
          };
        }
        if (token !== "deadbeef") {
          return {
            status: 401,
            body: JSON.stringify({
              error: "Invalid JWT token",
            }),
          };
        }
        return {
          status: 200,
          body: JSON.stringify({
            id: 1,
            jsonrpc: "2.0",
            result: hashStr,
          }),
        };
      });
    });

    it("token is missing", async () => {
      await expect(
        createVlayerClient().prove({
          address: `0x${"a".repeat(40)}`,
          functionName: "main",
          proverAbi: [],
          args: [],
          chainId: 42,
        }),
      ).rejects.toThrowError(
        `Missing JWT token${VLAYER_ERROR_NOTES[HttpAuthorizationError.name]}`,
      );
    });

    it("token is invalid", async () => {
      await expect(
        createVlayerClient({ token: "beefdead " }).prove({
          address: `0x${"a".repeat(40)}`,
          functionName: "main",
          proverAbi: [],
          args: [],
          chainId: 42,
        }),
      ).rejects.toThrowError(
        `Invalid JWT token${VLAYER_ERROR_NOTES[HttpAuthorizationError.name]}`,
      );
    });
  });
});
