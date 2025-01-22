import { describe, expect, test } from "vitest";
import { DnsResolver } from "./dnsResolver";

const resolver = new DnsResolver();

describe("resolveDkimDns Integration", () => {
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
