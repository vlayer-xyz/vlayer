import { toByteArray } from "base64-js";
import { toHex } from "viem";
import { handleAuthErrors } from "../lib/handleErrors";

interface DnsResponse {
  Status: number;
  TC: boolean;
  RD: boolean;
  RA: boolean;
  AD: boolean;
  CD: boolean;
  Question: {
    name: string;
    type: number;
  }[];
  Answer:
    | {
        name: string;
        type: number;
        TTL: number;
        data: string;
      }[]
    | undefined;
  VerificationData:
    | {
        valid_until: number;
        signature: string;
        pub_key: string;
      }
    | undefined;
}

function parseBase64(data: string): `0x${string}` {
  return toHex(toByteArray(data));
}

function requireNonnegativeInteger(value: number, fieldName: string): bigint {
  if (!Number.isInteger(value) || value < 0) {
    throw new Error(`${fieldName} must be a nonnegative integer`);
  }
  return BigInt(value);
}

function parseVerificationData(response: DnsResponse) {
  if (!response.VerificationData) {
    console.warn(`No verification data in DNS response`);
    return {
      validUntil: 0n,
      signature: "0x" as const,
      pubKey: "0x" as const,
    };
  }
  return {
    validUntil: requireNonnegativeInteger(
      response.VerificationData.valid_until,
      "DNS verification valid_until",
    ),
    signature: parseBase64(response.VerificationData.signature),
    pubKey: parseBase64(response.VerificationData.pub_key),
  };
}

function takeLastAnswer(response: DnsResponse) {
  const answer = response.Answer;
  if (!answer || answer?.length == 0) {
    throw new Error(
      `No DNS answer found\n${JSON.stringify(response, null, 2)}`,
    );
  }
  const record = answer.flat().at(-1)!;
  return {
    recordType: record.type,
    ttl: requireNonnegativeInteger(record.TTL, "DNS record TTL"),
    ...record,
  };
}

export class DnsResolver {
  constructor(
    private host: string,
    private token?: string,
  ) {}

  async resolveDkimDns(selector: string, domain: string) {
    const headers: Record<string, string> = {
      Accept: "application/dns-json",
    };
    if (this.token !== undefined) {
      headers["Authorization"] = `Bearer ${this.token}`;
    }

    const rawResponse = await fetch(
      `${this.host}?name=${selector}._domainkey.${domain}&type=TXT`,
      {
        headers,
      },
    );
    const responseJson = await rawResponse.json();

    if (!rawResponse.ok) {
      throw handleAuthErrors(rawResponse.status, responseJson);
    }

    const response = responseJson as DnsResponse;

    return {
      dnsRecord: takeLastAnswer(response),
      verificationData: parseVerificationData(response),
    };
  }
}
