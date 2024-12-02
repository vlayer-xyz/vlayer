import { describe, expect, test } from "vitest";
import { readFile } from "testHelpers/readFile";
import { preverifyEmail } from "./preverify";

describe("Preverify email: integration", () => {
  test("adds dns record to email mime", async () => {
    const rawEmail = readFile("./src/api/email/testdata/test_email.txt");
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
      return `DKIM-Signature: v=1; a=rsa-sha256; d=${domain}; s=selector; c=relaxed/relaxed; q=dns/txt; bh=; h=From:Subject:Date:To; b=\n${email}`;
    }

    test("looks for DKIM header with the domain matching the sender and removes all other DKIM headers", async () => {
      const emailWithOneDkimHeader = readFile(
        "./src/api/email/testdata/test_email.txt",
      );
      const emailWithAddedHeaders = addDkimWithDomain(
        "example.com",
        emailWithOneDkimHeader,
      );
      const email = await preverifyEmail(emailWithAddedHeaders);

      expect(email.email).toMatch(/^DKIM-Signature.*\nx-DKIM-Signature/);
      expect(email.dnsRecords).toStrictEqual(["v=DKIM1; p="]);
    });

    test("throws error if no DNS record domain matches the sender", async () => {
      const emailWithOneDkimHeader = readFile(
        "./src/api/email/testdata/test_email.txt",
      );
      const emailWithAddedHeaders = addDkimWithDomain(
        "otherdomain.com",
        emailWithOneDkimHeader,
      );
      await expect(preverifyEmail(emailWithAddedHeaders)).rejects.toThrow(
        "Found 0 DKIM headers matching the sender domain",
      );
    });

    test("throws error if multiple DNS record domains match the sender", async () => {
      const emailWithOneDkimHeader = readFile(
        "./src/api/email/testdata/test_email.txt",
      );
      let emailWithAddedHeaders = addDkimWithDomain(
        "example.com",
        emailWithOneDkimHeader,
      );
      emailWithAddedHeaders = addDkimWithDomain(
        "example.com",
        emailWithAddedHeaders,
      );
      await expect(preverifyEmail(emailWithAddedHeaders)).rejects.toThrow(
        "Found 2 DKIM headers matching the sender domain",
      );
    });
  });
});
