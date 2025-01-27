export function getStringPaths(jsonString: string): string[] {
  const paths: string[] = [];

  function traverse(obj: Record<string, unknown>, currentPath = "") {
    if (typeof obj === "object" && obj !== null) {
      for (const key in obj) {
        const newPath = currentPath ? `${currentPath}.${key}` : key;
        if (typeof obj[key] === "string") {
          paths.push(newPath);
        }
        traverse(obj[key] as Record<string, unknown>, newPath);
      }
    }
  }

  const parsed = JSON.parse(jsonString) as Record<string, unknown>;

  traverse(parsed);

  return paths;
}
