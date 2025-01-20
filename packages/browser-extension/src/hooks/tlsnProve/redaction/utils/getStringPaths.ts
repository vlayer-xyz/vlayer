import { Utf8String } from "./utf8String";
import { InvalidJsonError } from "./error";

export function getStringPaths(jsonString: Utf8String): string[] {
  const paths: string[] = [];

  function traverse(obj: unknown, currentPath = ""): void {
    if (typeof obj !== "object" || obj === null) {
      return;
    }

    for (const [key, value] of Object.entries(obj)) {
      const newPath = currentPath ? `${currentPath}.${key}` : key;

      if (typeof value === "string") {
        paths.push(newPath);
      }
      traverse(value, newPath);
    }
  }

  let parsed: unknown;
  try {
    parsed = JSON.parse(jsonString.toUtf16String());
  } catch (e) {
    throw new InvalidJsonError((e as Error).message);
  }

  traverse(parsed);
  return paths;
}
