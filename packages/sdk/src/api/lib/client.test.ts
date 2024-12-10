import {
  describe,
  expect,
  it,
  vi,
  beforeEach,
  beforeAll,
  type MockInstance,
} from "vitest";

import { createExtensionWebProofProvider } from "../webProof";
import { createVlayerClient } from "./client";
import { type BrandedHash, type VlayerClient } from "types/vlayer";
import { ZkProvingStatus } from "src/web-proof-commons";
import createFetchMock from "vitest-fetch-mock";

declare const global: {
  chrome: object;
};

const fetchMocker = createFetchMock(vi);
fetchMocker.enableMocks();

beforeEach(() => {
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

function generateRandomHash() {
  let hash = "0x";
  for (let i = 0; i < 40; ++i) {
    hash += Math.floor(Math.random() * 16).toString(16);
  }
  return hash;
}

describe("Success zk-proving", () => {
  let hashStr: string;
  let zkProvingSpy: MockInstance<(status: ZkProvingStatus) => void>;
  let vlayer: VlayerClient;

  beforeAll(() => {
    hashStr = generateRandomHash();
    const webProofProvider = createExtensionWebProofProvider();
    zkProvingSpy = vi.spyOn(webProofProvider, "notifyZkProvingStatus");
    vlayer = createVlayerClient({ webProofProvider });
  });
  it("should send message to extension that zkproving started", async () => {
    fetchMocker.mockResponseOnce(() => {
      return {
        body: JSON.stringify({
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
    expect(zkProvingSpy).toBeCalledTimes(2);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(1, ZkProvingStatus.Proving);
  });
  it("should send message to extension that zkproving is done", async () => {
    fetchMocker.mockResponseOnce(() => {
      return {
        body: JSON.stringify({
          result: {
            status: "done",
            data: {},
          },
        }),
      };
    });

    const hash = { hash: hashStr } as BrandedHash<[], string>;
    await vlayer.waitForProvingResult({ hash });

    expect(zkProvingSpy).toBeCalledTimes(2);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(2, ZkProvingStatus.Done);
  });
});

describe("Failed zk-proving", () => {
  beforeEach(() => {
    fetchMocker.mockResponseOnce((req) => {
      if (req.url === "http://127.0.0.1:3000/") {
        return {
          status: 500,
        };
      }
      return {};
    });
  });
  it("should send message to extension that zkproving started and then that failed ", async () => {
    const webProofProvider = createExtensionWebProofProvider();

    const zkProvingSpy = vi.spyOn(webProofProvider, "notifyZkProvingStatus");

    const vlayer = createVlayerClient({ webProofProvider });
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
      console.log("Error waiting for proving result", e);
    }

    expect(zkProvingSpy).toBeCalledTimes(2);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(1, ZkProvingStatus.Proving);
    expect(zkProvingSpy).toHaveBeenNthCalledWith(2, ZkProvingStatus.Error);
  });
});
