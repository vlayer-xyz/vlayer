import { Commit } from "tlsn-wasm";
import { match, P } from "ts-pattern";
import { calculateRequestRedactionRanges as calculateRequestRanges } from "./tlsn.request.ranges";
import { calculateResponseRedactionRanges as calculateResponseRanges } from "./tlsn.response.ranges";
import { RedactionConfig } from "src/web-proof-commons/types/message";
import { CommitData } from "tlsn-js/src/types";
import { InvalidRangeError, OutOfBoundsError } from "./tlsn.ranges.error";
import { toUtf8Transcript, Transcript, Utf8Transcript } from "./utils";

const emptyCommit: Commit = {
  sent: [],
  recv: [],
};

export function calcRedactionRanges(
  transcript: Utf8Transcript,
  redactionConfig: RedactionConfig,
) {
  return redactionConfig.reduce((commit, redactionItem) => {
    return match(redactionItem)
      .with({ response: P.any }, (responseRedactionItem) => ({
        ...commit,
        recv: [
          ...commit.recv,
          ...calculateResponseRanges(responseRedactionItem, transcript.recv),
        ],
      }))
      .with({ request: P.any }, (requestRedactionItem) => ({
        ...commit,
        sent: [
          ...commit.sent,
          ...calculateRequestRanges(requestRedactionItem, transcript.sent),
        ],
      }))
      .exhaustive();
  }, emptyCommit);
}

export const calcRevealRanges = (
  wholeTranscriptRange: CommitData,
  redactRanges: CommitData[],
): CommitData[] => {
  const validatedRedactRanges = redactRanges
    .map((range) => validateRange(range, wholeTranscriptRange))
    .sort((a, b) => a.start - b.start);

  const hasOverlap = validatedRedactRanges.find(
    (range, index) =>
      index > 0 && range.start <= validatedRedactRanges[index - 1].end,
  );

  if (hasOverlap) {
    throw new InvalidRangeError(hasOverlap);
  }

  const result: CommitData[] = [];
  let begin = wholeTranscriptRange.start;

  for (const redactRange of validatedRedactRanges) {
    result.push({ start: begin, end: redactRange.start });
    begin = redactRange.end;
  }

  result.push(
    validateRange(
      {
        start: begin,
        end: wholeTranscriptRange.end,
      },
      wholeTranscriptRange,
    ),
  );

  return result.filter((range) => range.start !== range.end);
};

const validateRange = (range: CommitData, wholeTranscriptRange: CommitData) => {
  if (range.start > range.end) {
    throw new InvalidRangeError(range);
  }
  if (
    range.start < wholeTranscriptRange.start ||
    range.end > wholeTranscriptRange.end
  ) {
    throw new OutOfBoundsError(range);
  }
  return range;
};

export function redact(
  transcript: Transcript,
  redactionConfig: RedactionConfig,
) {
  // transform the transcript to utf8
  const utf8Transcript = toUtf8Transcript(transcript);
  // calculate the redaction ranges
  const redactionRanges = calcRedactionRanges(utf8Transcript, redactionConfig);
  // we are expected to return the reveal ranges
  return {
    sent: calcRevealRanges(
      utf8Transcript.sent.message.range,
      redactionRanges.sent,
    ),
    recv: calcRevealRanges(
      utf8Transcript.recv.message.range,
      redactionRanges.recv,
    ),
  };
}
