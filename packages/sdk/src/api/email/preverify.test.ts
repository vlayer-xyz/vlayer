import { describe, expect, test, it, vi, beforeEach } from "vitest";
import { readFile } from "testHelpers/readFile";
import { findIndicesOfMatchingDomains, preverifyEmail } from "./preverify";
import createFetchMock from "vitest-fetch-mock";
import { HttpAuthorizationError, VLAYER_ERROR_NOTES } from "../lib/errors";

const fetchMocker = createFetchMock(vi);

const rawEmail = readFile("./src/api/email/testdata/test_email.txt");

describe("Preverify email: integration", () => {
  test("adds dns record to email mime", async () => {
    const preverifiedEmail = await preverifyEmail({
      mimeEmail: rawEmail,
      dnsResolverUrl: "https://dns.google/resolve",
    });
    expect(preverifiedEmail.email).toBe(rawEmail);
    expect(preverifiedEmail.dnsRecord).toMatchObject({
      name: "20230601._domainkey.google.com.",
      recordType: 16,
      ttl: expect.any(BigInt), // eslint-disable-line @typescript-eslint/no-unsafe-assignment
      data: expect.stringContaining("v=DKIM1; k=rsa; p="), // eslint-disable-line @typescript-eslint/no-unsafe-assignment
    });
    expect(preverifiedEmail.dnsRecord.data).toContain("v=DKIM1; k=rsa; p=");
  });

  test("throws error if DKIM not found", async () => {
    const emailWithNoDkimHeader = 'From: "Alice"\n\nBody';
    await expect(
      preverifyEmail({
        mimeEmail: emailWithNoDkimHeader,
        dnsResolverUrl: "https://dns.google/resolve",
      }),
    ).rejects.toThrow("No DKIM header found");
  });

  test("throws error if DNS could not be resolved", async () => {
    const emailWithNoDkimHeader = readFile(
      "./src/api/email/testdata/test_email_unknown_domain.txt",
    );
    await expect(
      preverifyEmail({
        mimeEmail: emailWithNoDkimHeader,
        dnsResolverUrl: "https://dns.google/resolve",
      }),
    ).rejects.toThrow();
  });

  describe("multiple DKIM headers", () => {
    function addDkimWithDomain(domain: string, email: string) {
      return `DKIM-Signature: v=1; a=rsa-sha256; d=${domain};
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
${email}`;
    }

    function addFakeDkimWithDomain(domain: string, email: string) {
      return `X-${addDkimWithDomain(domain, email)}`;
    }

    test("looks for DKIM header with the domain matching the sender and removes all other DKIM headers", async () => {
      const emailWithAddedHeaders = ["example.com", "hello.kitty"].reduce(
        (email, domain) => addDkimWithDomain(domain, email),
        rawEmail,
      );
      const email = await preverifyEmail({
        mimeEmail: emailWithAddedHeaders,
        dnsResolverUrl: "https://dns.google/resolve",
      });
      expect(
        email.email
          .startsWith(`X-DKIM-Signature: v=1; a=rsa-sha256; d=hello.kitty;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
X-DKIM-Signature: v=1; a=rsa-sha256; d=example.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
DKIM-Signature: a=rsa-sha256; bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=;\r
 c=simple/simple; d=google.com;`),
      ).toBeTruthy();
      expect(email.dnsRecord.data).toEqual(
        "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA4zd3nfUoLHWFbfoPZzAb8bvjsFIIFsNypweLuPe4M+vAP1YxObFxRnpvLYz7Z+bORKLber5aGmgFF9iaufsH1z0+aw8Qex7uDaafzWoJOM/6lAS5iI0JggZiUkqNpRQLL7H6E7HcvOMC61nJcO4r0PwLDZKwEaCs8gUHiqRn/SS3wqEZX29v/VOUVcI4BjaOzOCLaz7V8Bkwmj4Rqq4kaLQQrbfpjas1naScHTAmzULj0Rdp+L1vVyGitm+dd460PcTIG3Pn+FYrgQQo2fvnTcGiFFuMa8cpxgfH3rJztf1YFehLWwJWgeXTriuIyuxUabGdRQu7vh7GrObTsHmIHwIDAQAB",
      );
    });

    test("throws error if no DNS record domain matches the sender", async () => {
      const emailWithOneDkimHeader = readFile(
        "./src/api/email/testdata/test_email_unknown_domain.txt",
      );
      const emailWithAddedHeaders = addDkimWithDomain(
        "otherdomain.com",
        emailWithOneDkimHeader,
      );
      await expect(
        preverifyEmail({
          mimeEmail: emailWithAddedHeaders,
          dnsResolverUrl: "https://dns.google/resolve",
        }),
      ).rejects.toThrow("Found 0 DKIM headers matching the sender domain");
    });

    test("removes signatures from subdomain signers", async () => {
      const email = await preverifyEmail({
        mimeEmail: addDkimWithDomain("subdomain.google.com", rawEmail),
        dnsResolverUrl: "https://dns.google/resolve",
      });
      expect(
        email.email
          .startsWith(`X-DKIM-Signature: v=1; a=rsa-sha256; d=subdomain.google.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
DKIM-Signature: a=rsa-sha256; bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=;\r
 c=simple/simple; d=google.com;`),
      ).toBeTruthy();
    });

    test("removes signatures with mismatching subdomains", async () => {
      const emailWithAddedHeaders = addDkimWithDomain(
        "subdomain.google.com",
        readFile("./src/api/email/testdata/test_email_subdomain.txt"),
      );
      await expect(
        preverifyEmail({
          mimeEmail: emailWithAddedHeaders,
          dnsResolverUrl: "https://dns.google/resolve",
        }),
      ).rejects.toThrow("Found 0 DKIM headers matching the sender domain");
    });

    test("throws error if multiple DNS record domains match the sender", async () => {
      let emailWithAddedHeaders = addDkimWithDomain("google.com", rawEmail);
      emailWithAddedHeaders = addDkimWithDomain(
        "google.com",
        emailWithAddedHeaders,
      );
      await expect(
        preverifyEmail({
          mimeEmail: emailWithAddedHeaders,
          dnsResolverUrl: "https://dns.google/resolve",
        }),
      ).rejects.toThrow("Found 3 DKIM headers matching the sender domain");
    });

    test("ignores x-dkim-signature headers", async () => {
      const emailWithPrefixedDkim = addFakeDkimWithDomain(
        "example.com",
        addFakeDkimWithDomain("example.com", rawEmail),
      );
      const email = await preverifyEmail({
        mimeEmail: emailWithPrefixedDkim,
        dnsResolverUrl: "https://dns.google/resolve",
      });
      expect(
        email.email
          .startsWith(`X-DKIM-Signature: v=1; a=rsa-sha256; d=example.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
X-DKIM-Signature: v=1; a=rsa-sha256; d=example.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
DKIM-Signature: a=rsa-sha256; bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=;\r
 c=simple/simple; d=google.com;`),
      ).toBeTruthy();
    });

    test("ignores dkim-signature somewhere inside a header", async () => {
      const headerWithDkim = `WTF-IS-THIS-HEADER: DKIM-SIGNATURE;`;
      const emailWithDkimInHeader = `${headerWithDkim}\n${addDkimWithDomain("example.com", rawEmail)}`;
      const email = await preverifyEmail({
        mimeEmail: emailWithDkimInHeader,
        dnsResolverUrl: "https://dns.google/resolve",
      });
      expect(
        email.email.startsWith(`WTF-IS-THIS-HEADER: DKIM-SIGNATURE;
X-DKIM-Signature: v=1; a=rsa-sha256; d=example.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
DKIM-Signature: a=rsa-sha256; bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=;\r
 c=simple/simple; d=google.com;`),
      ).toBeTruthy();
    });

    test("ignores dkim-signature somewhere inside a body", async () => {
      const emailWithAddedDkim = addDkimWithDomain("example.com", rawEmail);
      const emailWithDkimsInBody = `${emailWithAddedDkim}\ndkim-signature   dkim-signature\r\ndkim-signature`;
      const email = await preverifyEmail({
        mimeEmail: emailWithDkimsInBody,
        dnsResolverUrl: "https://dns.google/resolve",
      });
      expect(
        email.email
          .startsWith(`X-DKIM-Signature: v=1; a=rsa-sha256; d=example.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
DKIM-Signature: a=rsa-sha256; bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=;\r
 c=simple/simple; d=google.com;`),
      ).toBeTruthy();
      expect(
        email.email.endsWith(
          `\ndkim-signature   dkim-signature\r\ndkim-signature`,
        ),
      ).toBeTruthy();
    });
  });
});

describe("findIndicesOfMatchingDomains", () => {
  test("returns indices of matching domains", () => {
    const signers = [
      { domain: "example.com", selector: "selector1" },
      { domain: "other.other", selector: "selector2" },
      { domain: "example.com", selector: "selector3" },
    ];
    expect(
      findIndicesOfMatchingDomains(signers, "some@example.com"),
    ).toStrictEqual([0, 2]);
  });

  test("returns empty array if no matching domains", () => {
    const signers = [
      { domain: "example.com", selector: "selector1" },
      { domain: "example.org", selector: "selector2" },
    ];
    expect(findIndicesOfMatchingDomains(signers, "other.other")).toStrictEqual(
      [],
    );
  });
});

describe("fails with readable error if", () => {
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

  it("token is missing", async () => {
    await expect(
      preverifyEmail({
        mimeEmail: rawEmail,
        dnsResolverUrl: "https://dns.google/resolve",
      }),
    ).rejects.toThrowError(
      `Missing JWT token${VLAYER_ERROR_NOTES[HttpAuthorizationError.name]}`,
    );
  });

  it("token is invalid", async () => {
    await expect(
      preverifyEmail({
        mimeEmail: rawEmail,
        dnsResolverUrl: "https://dns.google/resolve",
        token: "beefdead",
      }),
    ).rejects.toThrowError(
      `Invalid JWT token${VLAYER_ERROR_NOTES[HttpAuthorizationError.name]}`,
    );
  });
});
