import { MessageTranscript } from "./types";
import { Utf8String } from "./utf8String";

export function getRequestUrl(transcript: MessageTranscript): {
  url: Utf8String;
  url_offset: number;
} {
  const url = transcript.message.content.split(" ")[1];
  if (!url) {
    throw new Error("No URL found in the transcript");
  }
  return {
    url,
    url_offset: transcript.message.content.indexOf(url),
  };
}

export function getQueryParams(path: string): string[] {
  return [...new Set(Array.from(new URL(path).searchParams.keys()))];
}
