import { describe, expect, test } from "vitest";
import { readFile } from "testHelpers/readFile";
import { findIndicesOfMatchingDomains, preverifyEmail } from "./preverify";

const rawEmail = readFile("./src/api/email/testdata/test_email.txt");

describe("Preverify email: integration", () => {
  test("adds dns record to email mime", async () => {
    const preverifiedEmail = await preverifyEmail(rawEmail);
    expect(preverifiedEmail).toMatchObject({
      email: rawEmail,
      dnsRecords: [expect.stringContaining("v=DKIM1; k=rsa; p=")],
    });
  });

  test("throws error if DKIM not found", async () => {
    const emailWithNoDkimHeader = 'From: "Alice"\n\nBody';
    await expect(preverifyEmail(emailWithNoDkimHeader)).rejects.toThrow(
      "No DKIM header found",
    );
  });

  test("throws error if DNS could not be resolved", async () => {
    const emailWithNoDkimHeader = readFile(
      "./src/api/email/testdata/test_email_unknown_domain.txt",
    );
    await expect(preverifyEmail(emailWithNoDkimHeader)).rejects.toThrow();
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
      const emailWithAddedHeaders = [
        "example.com",
        "hello.kitty",
        "google.com",
      ].reduce((email, domain) => addDkimWithDomain(domain, email), rawEmail);
      const email = await preverifyEmail(emailWithAddedHeaders);
      expect(
        email.email
          .startsWith(`X-DKIM-Signature: v=1; a=rsa-sha256; d=google.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
X-DKIM-Signature: v=1; a=rsa-sha256; d=hello.kitty;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
DKIM-Signature: v=1; a=rsa-sha256; d=example.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
X-DKIM-Signature: a=rsa-sha256; bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=;\r
 c=simple/simple; d=google.com;`),
      ).toBeTruthy();
      expect(email.dnsRecords).toStrictEqual(["v=DKIM1; p="]);
    });

    test("throws error if no DNS record domain matches the sender", async () => {
      const emailWithAddedHeaders = addDkimWithDomain(
        "otherdomain.com",
        rawEmail,
      );
      await expect(preverifyEmail(emailWithAddedHeaders)).rejects.toThrow(
        "Found 0 DKIM headers matching the sender domain",
      );
    });

    test("throws error if multiple DNS record domains match the sender", async () => {
      let emailWithAddedHeaders = addDkimWithDomain("example.com", rawEmail);
      emailWithAddedHeaders = addDkimWithDomain(
        "example.com",
        emailWithAddedHeaders,
      );
      await expect(preverifyEmail(emailWithAddedHeaders)).rejects.toThrow(
        "Found 2 DKIM headers matching the sender domain",
      );
    });

    test("ignores x-dkim-signature headers", async () => {
      const emailWithPrefixedDkim = addDkimWithDomain(
        "example.com",
        addFakeDkimWithDomain("example.com", rawEmail),
      );
      const email = await preverifyEmail(emailWithPrefixedDkim);
      expect(
        email.email
          .startsWith(`DKIM-Signature: v=1; a=rsa-sha256; d=example.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
X-DKIM-Signature: v=1; a=rsa-sha256; d=example.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
X-DKIM-Signature: a=rsa-sha256; bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=;\r
 c=simple/simple; d=google.com;`),
      ).toBeTruthy();
    });

    test("ignores dkim-signature somewhere inside a header", async () => {
      const headerWithDkim = `WTF-IS-THIS-HEADER: DKIM-SIGNATURE;`;
      const emailWithDkimInHeader = `${headerWithDkim}\n${addDkimWithDomain("example.com", rawEmail)}`;
      const email = await preverifyEmail(emailWithDkimInHeader);
      expect(
        email.email.startsWith(`WTF-IS-THIS-HEADER: DKIM-SIGNATURE;
DKIM-Signature: v=1; a=rsa-sha256; d=example.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
X-DKIM-Signature: a=rsa-sha256; bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=;\r
 c=simple/simple; d=google.com;`),
      ).toBeTruthy();
    });

    test("ignores dkim-signature somewhere inside a body", async () => {
      const emailWithAddedDkim = addDkimWithDomain("example.com", rawEmail);
      const emailWithDkimsInBody = `${emailWithAddedDkim}\ndkim-signature   dkim-signature\r\ndkim-signature`;
      const email = await preverifyEmail(emailWithDkimsInBody);
      expect(
        email.email
          .startsWith(`DKIM-Signature: v=1; a=rsa-sha256; d=example.com;
 s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=
X-DKIM-Signature: a=rsa-sha256; bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=;\r
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
    expect(findIndicesOfMatchingDomains(signers, "example.com")).toStrictEqual([
      0, 2,
    ]);
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
