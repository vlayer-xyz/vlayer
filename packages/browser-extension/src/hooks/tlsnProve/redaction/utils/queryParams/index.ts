import { MessageTranscript } from "hooks/tlsnProve/redaction/types";
import { EncodedString } from "../encodeString";

export function findUrlInRequest(transcript: MessageTranscript): {
  url: EncodedString;
  url_offset: number;
} {
  const url = transcript.message.content.split(" ")[1];
  return {
    url,
    url_offset: transcript.message.content.indexOf(url),
  };
}

export function findAllQueryParams(path: string): string[] {
  return [...new Set(Array.from(new URL(path).searchParams.keys()))];
}
