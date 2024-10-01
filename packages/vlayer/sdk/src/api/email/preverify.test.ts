import { describe, expect, test } from "vitest";
import { preverifyEmail } from "./preverify.ts";
import { readFile } from "../../testHelpers/readFile";

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

  test("throws error if multiple DNS records found", async () => {
    const emailWithNoDkimHeader = readFile(
      "./src/api/email/testdata/test_email_multiple_dkims.txt",
    );
    await expect(preverifyEmail(emailWithNoDkimHeader)).rejects.toThrow(
      "Multiple DKIM headers found",
    );
  });
});
