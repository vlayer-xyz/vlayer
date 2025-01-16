import { Commit } from "tlsn-wasm";
import { match, P } from "ts-pattern";
import { calculateRequestRanges } from "./tlsn.request.ranges";
import { calculateResponseRanges } from "./tlsn.response.ranges";
import { RedactionConfig } from "src/web-proof-commons/types/message";
import { ParsedTranscriptData } from "tlsn-js";
import { CommitData } from "tlsn-js/src/types";
import { InvalidRangeError, OutOfBoundsError } from "./tlsn.ranges.error";
export type Transcript = {
  sent: string;
  recv: string;
  ranges: {
    recv: ParsedTranscriptData;
    sent: ParsedTranscriptData;
  };
};

const emptyCommit: Commit = {
  sent: [],
  recv: [],
};

export function calcRedactionRanges(
  transcript: Transcript,
  redactionConfig: RedactionConfig,
) {
  return redactionConfig.reduce((commit, redactionItem) => {
    return match(redactionItem)
      .with({ response: P.any }, (responseRedactionItem) => {
        return {
          ...commit,
          recv: [
            ...commit.recv,
            ...calculateResponseRanges(
              responseRedactionItem,
              transcript.recv,
              transcript.ranges.recv,
            ),
          ],
        };
      })
      .with({ request: P.any }, (requestRedactionItem) => {
        return {
          ...commit,
          sent: [
            ...commit.sent,
            ...calculateRequestRanges(
              requestRedactionItem,
              transcript.sent,
              transcript.ranges.sent,
            ),
          ],
        };
      })
      .exhaustive();
  }, emptyCommit);
}

export const calcRevealRanges = (
  wholeTranscriptRange: CommitData,
  redactRanges: CommitData[],
): CommitData[] => {
  const result: CommitData[] = [];
  let begin = wholeTranscriptRange.start;

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

  console.log(validatedRedactRanges);

  for (const redactRange of validatedRedactRanges) {
    result.push({ start: begin, end: redactRange.start });
    begin = redactRange.end;
  }

  const lastRange = validateRange(
    {
      start: begin,
      end: wholeTranscriptRange.end,
    },
    wholeTranscriptRange,
  );
  result.push(lastRange);

  return result.filter(differentStartAndEnd);
};

const differentStartAndEnd = (range: CommitData) => range.start !== range.end;

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
  const redactionRanges = calcRedactionRanges(transcript, redactionConfig);
  return {
    sent: calcRevealRanges(transcript.ranges.sent.all, redactionRanges.sent),
    recv: calcRevealRanges(transcript.ranges.recv.all, redactionRanges.recv),
  };
}
