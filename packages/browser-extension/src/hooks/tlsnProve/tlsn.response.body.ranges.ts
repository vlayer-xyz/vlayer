import { ParsedTranscriptData } from "tlsn-js";
import {
  BodyRangeNotFoundError,
  InvalidJsonError,
  InvalidPathError,
  NonStringValueError,
  PathNotFoundError,
} from "./tlsn.ranges.error";

export const validPathRegex = /^(\[\d+\]|[a-zA-Z_]\w*)(\.\w+|\[\d+\])*$/;

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
    //parse body json
    const bodyJson = getBodyJson(raw, transcriptRanges);
    // Validate path and get path segments
    const pathSegments = getPathSegments(path);

    // Initialize helper variables
    let currentObj = bodyJson;
    let currentPath = "";
    let valueStart = transcriptRanges.body.start;
    let valueEnd = transcriptRanges.body.end;
    let searchStartPos = valueStart;

    pathSegments.forEach((segment, i) => {
      //check if currentObj is undefined or null
      currentPath = getCurrentPath(
        currentObj,
        currentPath,
        segment.value,
        path,
      );

      // initialize keyPos
      let keyPos = 0;

      searchStartPos = raw.indexOf(JSON.stringify(currentObj));
      const isArrayIndex = segment.type === PathSegmentType.ArrayIndex;
      if (isArrayIndex) {
        let foundIndex = -1;
        while (foundIndex < parseInt(segment.value)) {
          keyPos = raw.indexOf(
            JSON.stringify((currentObj as object[])[foundIndex + 1]),
            searchStartPos + 1,
          );
          foundIndex++;
        }
      } else {
        keyPos = raw.indexOf(`"${segment.value}"`, searchStartPos);
        if (keyPos === -1) {
          throw new PathNotFoundError(path);
        }
      }

      currentObj = isArrayIndex
        ? (currentObj as object[])[parseInt(segment.value)]
        : (currentObj as Record<string, object>)[segment.value];

      // this is the last segment of the path
      if (i === pathSegments.length - 1) {
        // Make sure the value is a string
        if (typeof currentObj !== "object") {
          if (typeof currentObj !== "string") {
            throw new NonStringValueError(typeof currentObj);
          }
        }
        // Only set final value position for the last segment
        const valueStr = JSON.stringify(currentObj).replace(/"/g, "");

        // set valueStart and valueEnd for the last segment
        if (isArrayIndex) {
          valueStart = keyPos + 1;
        } else {
          valueStart = raw.indexOf(valueStr, keyPos + segment.value.length + 1);
        }
        valueEnd = valueStart + valueStr.length;
      }
    });

    return {
      start: valueStart,
      end: valueEnd,
    };
  });
};

export { calculateJsonBodyRanges, filterExceptPaths };

enum PathSegmentType {
  ArrayIndex = "arrayIndex",
  Key = "key",
}

const getBodyJson = (raw: string, transcriptRanges: ParsedTranscriptData) => {
  if (!transcriptRanges.body) {
    throw new BodyRangeNotFoundError();
  }
  let bodyJson;
  try {
    bodyJson = JSON.parse(
      raw.slice(transcriptRanges.body.start, transcriptRanges.body.end + 1),
    ) as object;
  } catch {
    throw new InvalidJsonError();
  }
  return bodyJson;
};

const getPathSegments = (path: string) => {
  if (!validPathRegex.test(path)) {
    throw new InvalidPathError(path);
  }
  //split path into segments and filter out empty strings
  const pathSegments = path.split(/[.[\]]/).filter(Boolean);
  return pathSegments.map((segment) => {
    return {
      value: segment,
      type: isNaN(Number(segment))
        ? PathSegmentType.Key
        : PathSegmentType.ArrayIndex,
    };
  });
};

const getCurrentPath = (
  currentObj: object,
  currentPath: string,
  segment: string,
  path: string,
) => {
  if (
    currentObj === undefined ||
    currentObj === null ||
    typeof currentObj !== "object"
  ) {
    throw new PathNotFoundError(path);
  }

  return currentPath ? `${currentPath}.${segment}` : segment;
};
