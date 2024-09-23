import { simpleParser, HeaderValue, type ParsedMail } from "mailparser";

export class DkimParsingError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "DkimParsingError";
  }
}

export async function parseEmail(mime: string) {
  return simpleParser(mime.trim(), {
    skipHtmlToText: true,
    skipTextToHtml: true,
    skipTextLinks: true,
    skipImageLinks: true,
  });
}

export function getDkimSigners(mail: ParsedMail) {
  const dkimHeader = mail.headers.get("dkim-signature");
  if (!dkimHeader) throw new DkimParsingError("No DKIM header found");
  if (Array.isArray(dkimHeader)) {
    return dkimHeader.map(parseHeader);
  }
  return [parseHeader(dkimHeader)];
}

function parseHeader(header: HeaderValue) {
  if (typeof header === "string" || !("params" in header)) {
    throw new DkimParsingError(`Invalid DKIM header ${header}`);
  }

  if (!header.params.d) {
    throw new DkimParsingError("DKIM header missing domain");
  }

  if (!header.params.s) {
    throw new DkimParsingError("DKIM header missing selector");
  }

  return {
    domain: header.params.d,
    selector: header.params.s,
  };
}
