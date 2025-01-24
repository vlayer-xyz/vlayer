import {
  InvalidJsonError,
  InvalidPathError,
  NonStringValueError,
  PathNotFoundError,
} from "../utils";

import { MessageTranscript } from "../types";
import { EncodedString } from "../utils";
export const validPathRegex = /^(\[\d+\]|[a-zA-Z_]\w*)(\.\w+|\[\d+\])*$/;

const filterExceptPaths = (except: string[], paths: string[]) => {
  return paths.filter((path) => !except.includes(path));
};

const calculateJsonBodyRanges = (
  rawMessage: MessageTranscript,
  paths: string[],
) => {
  return paths.map((path) => {
    const bodyRange = rawMessage.body.range;
    let bodyJson;
    try {
      //parse body json
      bodyJson = JSON.parse(rawMessage.body.content.toString()) as object;
    } catch {
      throw new InvalidJsonError(rawMessage.body.content.toString());
    }
    // Validate path and get path segments
    const pathSegments = getPathSegments(path);

    // Initialize helper variables
    let currentObj = bodyJson;
    let currentPath = "";
    let valueStart = bodyRange.start;
    let valueEnd = bodyRange.end;
    let searchStartPos = valueStart;

    pathSegments.forEach((segment, i) => {
      //check if currentObj is undefined or null
      currentPath = getCurrentPath(
        currentObj,
        currentPath,
        segment.value,
        path,
      );

      let keyPos = 0;

      searchStartPos = rawMessage.body.content.indexOf(
        JSON.stringify(currentObj),
      );
      const isArrayIndex = segment.type === PathSegmentType.ArrayIndex;
      if (isArrayIndex) {
        let foundIndex = -1;
        const alreadyChecked: number[] = [];
        while (foundIndex < parseInt(segment.value)) {
          const newKeyPos = rawMessage.message.content.indexOf(
            JSON.stringify((currentObj as object[])[foundIndex + 1]),
            searchStartPos + 1,
          );
          if (!alreadyChecked.includes(newKeyPos)) {
            keyPos = newKeyPos;
            alreadyChecked.push(newKeyPos);
          }
          foundIndex++;
        }
      } else {
        keyPos = rawMessage.message.content.indexOf(
          `"${segment.value}"`,
          searchStartPos,
        );
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
        if (typeof currentObj !== "string") {
          throw new NonStringValueError(typeof currentObj);
        }
        // Only set final value position for the last segment
        const valueStr = JSON.stringify(currentObj).replace(/"/g, "").trim();
        valueStart = rawMessage.message.content.indexOf(
          valueStr,
          keyPos + new EncodedString(segment.value, rawMessage.encoding).length,
        );
        valueEnd =
          valueStart + new EncodedString(valueStr, rawMessage.encoding).length;
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
