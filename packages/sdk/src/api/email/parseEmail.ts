import PostalMime, { type Email, type Header } from "postal-mime";

export class DkimParsingError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "DkimParsingError";
  }
}

export interface DkimDomainSelector {
  domain: string;
  selector: string;
}

export async function parseEmail(mime: string) {
  return await PostalMime.parse(mime.trim());
}

export function getDkimSigners(mail: Email): DkimDomainSelector[] {
  const dkimHeader = mail.headers.filter((h) => h.key === "dkim-signature");
  if (dkimHeader.length === 0) {
    throw new DkimParsingError("No DKIM header found");
  }
  return dkimHeader.map(parseHeader);
}

export function parseParams(str: string) {
  return Object.fromEntries(
    str.split(";").map((s) =>
      s
        .trim()
        .split("=")
        .map((v) => v && v.trim()),
    ),
  ) as Record<string, string>;
}

function parseHeader(header: Header): DkimDomainSelector {
  const params = parseParams(header.value);
  if (!params) {
    throw new DkimParsingError(`Invalid DKIM header ${header.value}`);
  }

  if (!params.d) {
    throw new DkimParsingError("DKIM header missing domain");
  }

  if (!params.s) {
    throw new DkimParsingError("DKIM header missing selector");
  }
  return {
    domain: params.d,
    selector: params.s,
  };
}
