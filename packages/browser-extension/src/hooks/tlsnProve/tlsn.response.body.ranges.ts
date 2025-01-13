import { ParsedTranscriptData } from "tlsn-js";
import {
  BodyRangeNotFoundError,
  InvalidPathError,
  NonStringValueError,
  PathNotFoundError,
} from "./tlsn.ranges.error";

const filterExceptPaths = (except: string[], paths: string[]) => {
  return paths.filter((path) => !except.includes(path));
};

const calculateJsonBodyRanges = (
  raw: string,
  transcriptRanges: ParsedTranscriptData,
  paths: string[],
) => {
  return paths.map((path) => {
    if (!transcriptRanges.body) {
      throw new BodyRangeNotFoundError();
    }
    const pathSegments = path.split(/[.[\]]/).filter(Boolean);
    let bodyJson;

    try {
      bodyJson = JSON.parse(
        raw.slice(transcriptRanges.body.start, transcriptRanges.body.end),
      ) as Record<string, unknown>;
    } catch {
      throw new InvalidPathError(path);
    }

    let currentObj: unknown = bodyJson;
    let currentPath = "";
    let valueStart = transcriptRanges.body.start;
    let valueEnd = transcriptRanges.body.end;
    let searchStartPos = valueStart;

    for (let i = 0; i < pathSegments.length; i++) {
      const segment = pathSegments[i];
      if (currentObj === undefined || currentObj === null) {
        throw new PathNotFoundError(path);
      }

      if (typeof currentObj === "object") {
        currentPath = currentPath ? `${currentPath}.${segment}` : segment;

        // Handle array indices differently
        const isArrayIndex = !isNaN(Number(segment));
        let keyPos;

        if (isArrayIndex) {
          // For array indices, we need to find the nth occurrence of an array element
          const arrayIndex = parseInt(segment);
          let foundIndex = -1;
          let pos = searchStartPos;

          while (foundIndex < arrayIndex) {
            pos = raw.indexOf("{", pos + 1);
            if (pos === -1) {
              throw new PathNotFoundError(path);
            }
            foundIndex++;
          }
          keyPos = pos;

          // If this is not the last segment, we want to start searching from this position
          if (i < pathSegments.length - 1) {
            searchStartPos = keyPos;
          }
        } else {
          keyPos = raw.indexOf(`"${segment}"`, searchStartPos);
          if (keyPos === -1) {
            throw new PathNotFoundError(path);
          }
        }

        currentObj = isArrayIndex
          ? (currentObj as unknown[])[parseInt(segment)]
          : (currentObj as Record<string, unknown>)[segment];

        if (i === pathSegments.length - 1) {
          // Make sure the value is a string
          if (typeof currentObj !== "object") {
            if (typeof currentObj !== "string") {
              throw new NonStringValueError(typeof currentObj);
            }
          }
          // Only set final value position for the last segment
          const valueStr = JSON.stringify(currentObj).replace(/"/g, "");

          if (isArrayIndex) {
            valueStart = keyPos + 1;
          } else {
            valueStart = raw.indexOf(valueStr, keyPos + segment.length + 1);
          }
          valueEnd = valueStart + valueStr.length;
        }
      } else {
        throw new PathNotFoundError(path);
      }
    }

    return {
      start: valueStart,
      end: valueEnd,
    };
  });
};

export { calculateJsonBodyRanges, filterExceptPaths };
