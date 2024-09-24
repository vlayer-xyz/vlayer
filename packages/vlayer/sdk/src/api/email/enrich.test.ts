import { describe, expect, test } from "vitest";
import fs from "fs";
import { enrichEmail } from "./enrich.ts";

describe("Enrich email: integration", () => {
  test("adds dns record to email mime", async () => {
    const rawEmail = fs
      .readFileSync("./src/api/email/testdata/test_email.txt")
      .toString();
    const initializedEmail = await enrichEmail(rawEmail);
    expect(initializedEmail).toMatchObject({
      email: rawEmail,
      dnsRecord: expect.stringContaining("v=DKIM1; k=rsa; p="),
    });
  });

  test("throws error if DKIM not found", async () => {
    const emailWithNoDkimHeader = 'From: "Alice"\n\nBody';
    await expect(enrichEmail(emailWithNoDkimHeader)).rejects.toThrow(
      "No DKIM header found",
    );
  });

  test("throws error if DNS could not be resolved", async () => {
    const emailWithNoDkimHeader = fs
      .readFileSync("./src/api/email/testdata/test_email_unknown_domain.txt")
      .toString();
    await expect(enrichEmail(emailWithNoDkimHeader)).rejects.toThrow(
      "queryTxt ENOTFOUND",
    );
  });
});
