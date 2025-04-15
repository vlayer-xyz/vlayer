import { describe, expect, test, it, vi, beforeEach } from "vitest";
import { DnsResolver } from "./dnsResolver";
import createFetchMock from "vitest-fetch-mock";

const fetchMocker = createFetchMock(vi);

describe("resolveDkimDns Integration", () => {
  const resolver = new DnsResolver("https://dns.google/resolve");

  test("resolves vlayer DNS", async () => {
    const resolved = await resolver.resolveDkimDns("google", "vlayer.xyz");
    const expected =
      "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoDLLSKLb3eyflXzeHwBz8qqg9mfpmMY+f1tp+HpwlEOeN5iHO0s4sCd2QbG2i/DJRzryritRnjnc4i2NJ/IJfU8XZdjthotcFUY6rrlFld20a13q8RYBBsETSJhYnBu+DMdIF9q3YxDtXRFNpFCpI1uIeA/x+4qQJm3KTZQWdqi/BVnbsBA6ZryQCOOJC3Ae0oodvz80yfEJUAi9hAGZWqRn+Mprlyu749uQ91pTOYCDCbAn+cqhw8/mY5WMXFqrw9AdfWrk+MwXHPVDWBs8/Hm8xkWxHOqYs9W51oZ/Je3WWeeggyYCZI9V+Czv7eF8BD/yF9UxU/3ZWZPM8EWKKQIDAQAB";
    expect(resolved.dnsRecord.data).toBe(expected);
  });

  test("throws error if dns not found", async () => {
    await expect(
      resolver.resolveDkimDns("abcd", "not-a-domain.com"),
    ).rejects.toThrow();
  });
});

describe("Authentication", () => {
  beforeEach(() => {
    fetchMocker.enableMocks();
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
        body: JSON.stringify({}),
      };
    });
  });

  it("requires passing a token", async () => {
    const userToken = "deadbeef";
    const resolver = new DnsResolver("http://localhost:3002", userToken);
    await expect(
      resolver.resolveDkimDns("google", "vlayer.xyz"),
    ).rejects.toThrowError("No DNS answer found");
  });

  describe("fails with an error if", () => {
    it("token is missing", async () => {
      const resolver = new DnsResolver("http://localhost:3002");
      await expect(
        resolver.resolveDkimDns("google", "vlayer.xyz"),
      ).rejects.toThrowError("Missing JWT token");
    });

    it("token is invalid", async () => {
      const resolver = new DnsResolver("http://localhost:3002", "beefdead");
      await expect(
        resolver.resolveDkimDns("google", "vlayer.xyz"),
      ).rejects.toThrowError("Invalid JWT token");
    });
  });
});
