import assert from "node:assert";
import { describe, expect, test } from "vitest";
import { getDkimSigners, parseEmail } from "./parseEmail.ts";
import { StructuredHeader } from "mailparser";
import PostalMime from "postal-mime";
const emailHeaders = `From: "John Doe" <john@d.oe>
To: "Jane Doe" <jane@d.oe>
Subject: Hello World
Date: Thu, 1 Jan 1970 00:00:00 +0000
`;

const dkimHeader =
  "DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=example.com; h=from:to:subject; s=selector1; b=abcdef;";

const body = "Hello, World!";

const emailFixture = `${emailHeaders}${dkimHeader}\n\n${body}`;

const dkimHeader2 =
  "DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=second.signer; h=from:to:subject; s=selector2; b=abcdef;";

const email = await parseEmail(
  `${emailHeaders}${dkimHeader}\n${dkimHeader2}\n\n${body}`,
);
const dkim = email.parsedHeaders.get(
  "dkim-signature",
)! as unknown as StructuredHeader[];

console.log("a", email);
describe("parseEmail", () => {
  test("should get dkim header from email", async () => {});

  test("correctly parses untrimmed email", async () => {
    const untrimmed = `\n   ${emailFixture}    \n`;
    const email = await parseEmail(untrimmed);
    console.log("email", email);
    expect(email.parsedHeaders.get("dkim-signature")).toBeDefined();
  });

  test("works well with multiple dkim headers", async () => {
    const dkimHeader2 =
      "DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=second.signer; h=from:to:subject; s=selector2; b=abcdef;";

    const email = await parseEmail(
      `${emailHeaders}${dkimHeader}\n${dkimHeader2}\n\n${body}`,
    );
    const dkim = email.parsedHeaders.get(
      "dkim-signature",
    )! as unknown as StructuredHeader[];

    console.log("dkim", dkim);
    expect(dkim).toHaveLength(2);
    expect(dkim[0].params.s).toBe("selector1");
    expect(dkim[1].params.s).toBe("selector2");
  });
});
//
describe("getDkimSigners", () => {
  test("should get dkim signers from email", async () => {
    const email = await parseEmail(emailFixture);
    const dkim = getDkimSigners(email);
    expect(dkim).toHaveLength(1);
    expect(dkim[0].domain).toBe("example.com");
    expect(dkim[0].selector).toBe("selector1");
  });

  test("should get multiple dkim signers from email", async () => {
    const dkimHeader2 =
      "DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=second.signer; h=from:to:subject; s=selector2; b=abcdef;";
    const email = await parseEmail(
      `${emailHeaders}${dkimHeader}\n${dkimHeader2}\n\n${body}`,
    );

    const dkim = getDkimSigners(email);
    expect(dkim).toHaveLength(2);
    expect(dkim[0].domain).toBe("example.com");
    expect(dkim[0].selector).toBe("selector1");
    expect(dkim[1].domain).toBe("second.signer");
    expect(dkim[1].selector).toBe("selector2");
  });

  test("should throw if no dkim header found", async () => {
    const email = await parseEmail(emailHeaders);
    expect(() => getDkimSigners(email)).toThrowError("No DKIM header found");
  });

  test("should throw if dkim header is invalid", async () => {
    const email = await parseEmail(
      `${emailHeaders}DKIM-Signature: invalid\n\n${body}`,
    );
    expect(() => getDkimSigners(email)).toThrowError(
      "DKIM header missing domain",
    );
  });

  test("should throw if dkim header is missing domain", async () => {
    const email = await parseEmail(
      `${emailHeaders}DKIM-Signature: v=1; s=selector\n\n${body}`,
    );
    expect(() => getDkimSigners(email)).toThrowError(
      "DKIM header missing domain",
    );
  });

  test("should throw if dkim header is missing selector", async () => {
    const email = await parseEmail(
      `${emailHeaders}DKIM-Signature: v=1; d=example.com\n\n${body}`,
    );
    expect(() => getDkimSigners(email)).toThrowError(
      "DKIM header missing selector",
    );
  });
});
