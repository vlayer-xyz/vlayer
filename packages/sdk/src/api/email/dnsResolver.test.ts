import { describe, expect, test } from "vitest";
import { resolveDkimDns } from "./dnsResolver";

describe("resolveDkimDns Integration", () => {
  test("resolves VLayer DNS", async () => {
    const resolved = await resolveDkimDns(
      "vlayer-xyz.20230601.gappssmtp.com",
      "20230601",
    );
    const expected =
      "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3gWcOhCm99qzN+h7/2+LeP3CLsJkQQ4EP/2mrceXle5pKq8uZmBl1U4d2Vxn4w+pWFANDLmcHolLboESLFqEL5N6ae7u9b236dW4zn9AFkXAGenTzQEeif9VUFtLAZ0Qh2eV7OQgz/vPj5IaNqJ7h9hpM9gO031fe4v+J0DLCE8Rgo7hXbNgJavctc0983DaCDQaznHZ44LZ6TtZv9TBs+QFvsy4+UCTfsuOtHzoEqOOuXsVXZKLP6B882XbEnBpXEF8QzV4J26HiAJFUbO3mAqZL2UeKC0hhzoIZqZXNG0BfuzOF0VLpDa18GYMUiu+LhEJPJO9D8zhzvQIHNrpGwIDAQAB";
    expect(resolved).toBe(expected);
  });

  test("resolves delegated dns", async () => {
    const resolved = await resolveDkimDns(
      "bolt.eu",
      "el7njvpsjxbr7wk7l7dss5ejzvijzoeu",
    );
    const expected =
      "v=DKIM1; k=rsa; p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDQxwOEYMZS2rPORBB94iL47Ute8zb1SUNl7K0zCQMk+M83AJHcwKjnJVhA4F0rLbSxY7cxJgl57lN4Vp5k10HHOil00oIn1S0ChBKHiFCQAMHCNonwDOdJa6mXwe2VwEM7hnVpRc/Eo0F0acpNMeYJxyLcTcOuZBNzcPm6t+4uTwIDAQAB";
    expect(resolved).toBe(expected);
  });

  test("throws error if dns not found", async () => {
    await expect(resolveDkimDns("not-a-domain.com", "abcd")).rejects.toThrow();
  });
});
