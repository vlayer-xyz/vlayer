import { Utf8String } from "./utf8String";

export type Transcript = {
  sent: string;
  recv: string;
};

export type MessagePartTranscript = {
  content: Utf8String;
  range: {
    start: number;
    end: number;
  };
};

export type MessageTranscript = {
  message: MessagePartTranscript;
  info: MessagePartTranscript;
  headers: MessagePartTranscript;
  body: MessagePartTranscript;
};

export type Utf8Transcript = {
  sent: MessageTranscript;
  recv: MessageTranscript;
};
