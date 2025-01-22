import { EncodedString, Encoding } from "./utils/encodeString";
import { type CommitData } from "tlsn-js/src/types";

export type Transcript = {
  sent: string;
  recv: string;
};

export type MessagePartTranscript = {
  content: EncodedString;
  range: CommitData;
};

export type MessageTranscript = {
  encoding: Encoding;
  message: MessagePartTranscript;
  info: MessagePartTranscript;
  headers: MessagePartTranscript;
  body: MessagePartTranscript;
};

export type EncodedTranscript = {
  sent: MessageTranscript;
  recv: MessageTranscript;
};
